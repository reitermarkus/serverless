require 'open-uri'
require 'socket'
require 'tempfile'

FUNCTIONS = %w[
  database
  devices
  filter
  log-data
  register-device
]

def dev?
  !ENV['PRODUCTION']
end

def swarm_active?
  swarm_state = `docker info --format '{{.Swarm.LocalNodeState}}'`.chomp
  swarm_state == 'active'
end

def create_secret(name, content)
  secret = Tempfile.new.tap { |s| s.write content }.tap(&:close)
  sh 'docker', 'secret', 'create', name, secret.path
end

namespace :deploy do
  desc 'deploy functions'
  task :functions, [:functions] do |task, args|
    functions = args.functions&.split

    unless functions
      functions = FUNCTIONS
      Rake::Task['deploy:swarm'].invoke unless swarm_active?
    end

    cd 'functions' do
      functions.each do |function|
        sh 'faas-cli', 'build', '-f', "#{function}.yml", function
        sh 'faas-cli', 'remove', '-f', "#{function}.yml", function
        sh 'faas-cli', 'deploy', '-f', "#{function}.yml", function
      end
    end
  end

  desc 'deploy swarm'
  task :swarm do
    user = 'admin'
    password = 'password'

    mongo_username = 'admin'
    mongo_password = 'password'

    unless swarm_active?
      sh 'docker', 'swarm', 'init'

      create_secret 'basic-auth-user', user
      create_secret 'basic-auth-password', password

      create_secret 'mongo-root-username', user
      create_secret 'mongo-root-password', password
    end

    hostname = Socket.gethostname
    puts "Setting Kafka hostname to “#{hostname}”…"
    ENV['KAFKA_PUBLIC_HOSTNAME'] = hostname

    ENV['ME_CONFIG_MONGODB_ADMINUSERNAME'] = mongo_username
    ENV['ME_CONFIG_MONGODB_ADMINPASSWORD'] = mongo_password

    ENV['BASIC_AUTH'] = dev? ? 'false' : 'true'

    mkdir_p 'faas'
    cp 'deploy.yml', 'faas/deploy.yml'

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
  end

  desc 'deploy swarm and functions'
  task :all => [:'deploy:swarm', :'deploy:functions']
end

task :default => :'deploy:all'

task :kill do
  sh 'docker', 'swarm', 'leave', '--force' if swarm_active?
end
