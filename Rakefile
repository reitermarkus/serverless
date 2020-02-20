require 'open-uri'
require 'open3'
require 'shellwords'
require 'socket'
require 'tempfile'
require 'timeout'
require 'yaml'
require 'net/http'
require 'json'
require 'etc'

def dev?
  !ENV['PRODUCTION']
end

def windows?
  !(RUBY_PLATFORM !~ /cygwin|mswin|mingw|bccwin|wince|emx/)
end

def swarm_active?
  stdout, *_ = Open3.capture3('docker', 'info', '--format', '{{.Swarm.LocalNodeState}}')
  swarm_state = stdout.chomp
  swarm_state == 'active'
end

def create_secret(name, content)
  secret = Tempfile.new.tap { |s| s.write content }.tap(&:close)
  sh 'docker', 'secret', 'create', name, secret.path
end

namespace :build do
  task :functions => :'build:ui' do |task, args|
    functions = args.extras

    if functions.empty?
      cd 'functions' do
        sh 'faas-cli', 'build', '--build-option', (dev? ? 'debug' : 'release'), '-f', 'functions.yml', '--parallel', Etc.nprocessors.to_s
      end
    else
      task.reenable
      functions.each do |function|
        cd 'functions' do
          sh 'faas-cli', 'build', '--build-option', (dev? ? 'debug' : 'release'), '-f', 'functions.yml', '--filter', function
        end
      end
    end
  end

  task :push => :'build:functions' do |task, args|
    functions = args.extras

    if functions.empty?
      cd 'functions' do
        sh 'faas-cli', 'push', '-f', 'functions.yml', '--parallel', Etc.nprocessors.to_s
      end
    else
      task.reenable
      functions.each do |function|
        cd 'functions' do
          sh 'faas-cli', 'push', '-f', 'functions.yml', '--filter', function
        end
      end
    end
  end

  task :ui do
    rm_rf 'functions/ui'
    cp_r 'marko', 'functions/ui'
    rm_rf 'functions/ui/dist'
    rm_rf 'functions/ui/node_modules'
  end
end

namespace :deploy do
  desc 'deploy functions'
  task :functions do |task, args|
    functions = args.extras

    if functions.empty?
      Rake::Task['deploy:swarm'].invoke unless swarm_active?

      cd 'functions' do
        sh 'faas-cli', 'remove', '-f', 'functions.yml'
        sh 'faas-cli', 'deploy', '-f', 'functions.yml'
      end
    else
      functions.each do |function|
        cd 'functions' do
          sh 'faas-cli', 'remove', '-f', 'functions.yml', '--filter', function
          sh 'faas-cli', 'deploy', '-f', 'functions.yml', '--filter', function
        end
      end
    end


  end

  desc 'deploy swarm'
  task :swarm => :'db:credentials' do
    user = 'admin'
    password = 'password'

    unless swarm_active?
      sh 'docker', 'swarm', 'init'

      create_secret 'basic-auth-user', user
      create_secret 'basic-auth-password', password

      create_secret 'mongo-root-username', ENV['ME_CONFIG_MONGODB_ADMINUSERNAME']
      create_secret 'mongo-root-password', ENV['ME_CONFIG_MONGODB_ADMINPASSWORD']
    end

    hostname = Socket.gethostname
    puts "Setting Kafka hostname to “#{hostname}”…"
    ENV['KAFKA_PUBLIC_HOSTNAME'] = hostname

    ENV['BASIC_AUTH'] = dev? ? 'false' : 'true'

    mkdir_p 'faas'
    deploy_yaml = YAML.safe_load(File.read('deploy.yml'))
    deploy_yaml['services']['mongo'].delete('volumes') if windows?
    File.write 'faas/deploy.yml', deploy_yaml.to_yaml

    prometheus_dir = 'faas/prometheus'
    mkdir_p prometheus_dir
    cd prometheus_dir do
      File.write 'alertmanager.yml', URI('https://raw.githubusercontent.com/openfaas/faas/master/prometheus/alertmanager.yml').open(&:read)
      File.write 'alert.rules.yml', URI('https://raw.githubusercontent.com/openfaas/faas/master/prometheus/alert.rules.yml').open(&:read)
      File.write 'prometheus.yml', URI('https://raw.githubusercontent.com/openfaas/faas/master/prometheus/prometheus.yml').open(&:read)
    end

    db_dir = File.expand_path('faas/db-data')
    mkdir_p db_dir
    ENV['DATABASE_DIR'] = db_dir

    puts 'Deploying stack…'
    sh 'docker', 'stack', 'deploy', '--compose-file', 'faas/deploy.yml', 'func'

    Rake::Task['db:restore'].invoke('faas/db-dump.gz') if windows? && File.exist?('faas/db-dump.gz')
  end
end

desc 'build functions'
task :build => :'build:functions'

desc 'deploy swarm and functions'
task :deploy => [:'deploy:swarm', :'deploy:functions']

task :default => :deploy

task :kill do
  next unless swarm_active?
  begin
    Rake::Task['db:dump'].invoke('faas/db-dump.gz')
  rescue
    $stderr.puts "Database not running, skipping dump."
  end
  sh 'docker', 'swarm', 'leave', '--force'
end

namespace :db do
  def mongo_container_id
    id = begin
      Timeout.timeout(30) do
        loop do
          stdout, _, status = Open3.capture3('docker', 'ps', '--filter', 'name=func_mongo\.', '--format', '{{.ID}}', '--latest')
          id = stdout.chomp
          break id if status.success? && !id.empty?
          sleep 0.5
        end
      end
    rescue
      raise 'No MongoDB container found.'
    end

    begin
      Timeout.timeout(30) do
        loop do
          stdout, _, status = Open3.capture3('docker', 'inspect', '--format', '{{.State.Health.Status}}', id)
          break if status.success? && stdout.chomp == 'healthy'
        end
      end
    rescue
      raise 'MongoDB container failed to start.'
    end

    id
  end

  task :credentials do
    mongo_username = 'admin'
    mongo_password = 'password'
    ENV['ME_CONFIG_MONGODB_ADMINUSERNAME'] = mongo_username
    ENV['ME_CONFIG_MONGODB_ADMINPASSWORD'] = mongo_password
  end

  desc 'dump database'
  task :dump, [:file] => :'db:credentials' do |task, args|
    file = args.file
    cmd = ['docker', 'exec', mongo_container_id, 'mongodump', '-u', ENV['ME_CONFIG_MONGODB_ADMINUSERNAME'], '-p', ENV['ME_CONFIG_MONGODB_ADMINPASSWORD'], '--authenticationDatabase', 'admin', '--gzip', '--archive']

    if file
      $stderr.puts "#{cmd.shelljoin} > #{file}"
      statuses = Open3.pipeline(cmd, out: file)
      create_shell_runner(cmd).call(statuses.all?(&:success?), statuses.last)
      task.reenable
    else
      sh *cmd
    end
  end

  desc 'restore database'
  task :restore, [:file] => :'db:credentials' do |task, args|
    file = args.file
    cmd = ['docker', 'exec', '-i', mongo_container_id, 'mongorestore', '-u', ENV['ME_CONFIG_MONGODB_ADMINUSERNAME'], '-p', ENV['ME_CONFIG_MONGODB_ADMINPASSWORD'], '--authenticationDatabase', 'admin', '--gzip', '--archive']

    if file
      $stderr.puts "#{cmd.shelljoin} < #{file}"
      statuses = Open3.pipeline(cmd, in: file)
      create_shell_runner(cmd).call(statuses.all?(&:success?), statuses.last)
      task.reenable
    else
      sh *cmd
    end
  end
end

DOCUMENTS = %w[presentation thesis].freeze

namespace :tex do
  DOCUMENTS.each do |doc|
    desc "compile #{doc}"
    task doc do |task, args|
      watch_arg = '-pvc' if args.extras.include?('watch')

      cd 'tex' do
        sh 'latexmk', '-cd', "#{doc}/#{doc}.tex", *watch_arg

        cd 'thesis' do
          sh 'ls', '-a'
          sh 'pwd'
        end
      end
    end
  end

  desc 'clean all documents'
  task :clean do
    cd 'tex' do
      sh 'latexmk', '-cd', '-C'
    end
  end

  task :download do
    json = Net::HTTP.get(URI('https://dev.azure.com/reitermarkus/serverless/_apis/build/builds?definitions=2&$top=1&api-version=5.0-preview.5'))
    parsed_json = JSON.parse(json)
    id = parsed_json.dig 'value', 0, 'id'
    download_url = "https://dev.azure.com/reitermarkus/9f00b2ca-5e57-4700-aee5-5e7c454ffc52/_apis/build/builds/#{id}/artifacts?artifactName=thesis&api-version=5.1&%24format=zip"

    if windows?
      sh 'wget', '-O', 'thesis.zip', download_url
    else
      File.write 'thesis.zip', URI(download_url).open(&:read)
    end

    sh 'unzip', 'thesis.zip'
    sh 'rm', '-rf', 'thesis.zip'
  end
end

desc 'compile all documents'
task :tex => DOCUMENTS.map { |doc| "tex:#{doc}" }
