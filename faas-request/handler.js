"use strict"
const MongoClient = require('mongodb').MongoClient
const Server      = require('mongodb').Server

const MONGO_HOST = process.env.MONGO_HOST || 'localhost'
const MONGO_PORT = process.env.MONGO_PORT || 27017
const MONGO_DB = process.env.MONGO_INITDB_DATABASE || 'sensor'
const MONGO_USER = process.env.MONGO_INITDB_ROOT_USERNAME || 'admin'
const MONGO_PASSWORD = process.env.MONGO_INITDB_ROOT_PASSWORD || 'root'

const MONGO_URL = `mongodb://${MONGO_USER}:${MONGO_PASSWORD}@${MONGO_HOST}:${MONGO_PORT}/${MONGO_DB}?authMechanism=DEFAULT&authSource=admin`
const client = new MongoClient(MONGO_URL, { poolSize: 10, useNewUrlParser: true })

var database

const prepareDB = () => {
  return new Promise((resolve, reject) => {
    if (database) {
      return resolve([database, " Database connection was cached."])
    }

    client.connect((err, db) => {
      if (err) {
        return reject(err)
      }

      database = db.db(MONGO_DB)

      return resolve([database, " Database connection was not cached."])
    })
  })
}

module.exports = (event, context) => {
  prepareDB().then(([db, message]) => {
    const value = event.body

    db.collection("request")
      .insertOne({ name: "req", value: value }, err => {
        if (err) {
          context.fail(err.toString())
        }

        context
          .status(200)
          .succeed(`Saved '${value}' in database.` + message);
      })
  }).catch(err => {
    context.fail(err.toString());
  })
}
