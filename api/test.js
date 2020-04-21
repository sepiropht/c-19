const MongoClient = require("mongodb").MongoClient;
const uri = process.env.MONGO_URL
const client = new MongoClient(uri, { useNewUrlParser: true });

module.exports = async (req, res) => {
  await client.connect()
  const db = client.db("death_france");
  const {
    query: { year },
  } = req;

  db.collection("person")
    .find({"datetime" : {"$gte": new Date(year)}})
    .toArray((error, documents) => {
      if (error) throw error;
      console.log(documents)
      res.send(documents);
    });
};
