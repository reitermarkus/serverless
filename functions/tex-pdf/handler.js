const unzipper = require('unzipper')
const axios = require('axios').default

const fetchPdf = (url, fileName) => new Promise(async (resolve, reject) => {
  try {
    const buffer = await axios.get(url, { responseType: 'arraybuffer' })
    const directory = await unzipper.Open.buffer(buffer.data)
    const file = directory.files.find(d => d.path === `thesis/${fileName}`)
    const fileBuffer = await file.buffer()
    resolve(fileBuffer)
  } catch (err) {
    reject(err.message)
  }
})

const getLatestBuildFromAzure = async () => {
  try {
    const json = await axios.get('https://dev.azure.com/reitermarkus/serverless/_apis/build/builds?definitions=2&$top=1&api-version=5.0-preview.5')
    const id = json.data.value[0].id
    const url = `https://dev.azure.com/reitermarkus/9f00b2ca-5e57-4700-aee5-5e7c454ffc52/_apis/build/builds/${id}/artifacts?artifactName=thesis&api-version=5.1&%24format=zip`
    return await fetchPdf(url, 'thesis.pdf')
  } catch (err) {
    return err
  }
}

module.exports = async (event, context) => {
  const headers = []
  headers['content-disposition'] = `inline; filename="thesis.pdf"`
  headers['Content-Type'] = 'application/pdf'

  const buffer = await getLatestBuildFromAzure()

  context
    .status(200)
    .headers(headers)
    .succeed(buffer)
}
