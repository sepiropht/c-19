const http = require("http");
const fs = require("fs");

const csv = require('csv-parser');
const res = []
fs.createReadStream('data.csv')
  .pipe(csv())
  .on('data', (row) => {

    console.log(row)
    //console.log(row.split(','))
  })
  .on('end', () => {
    console.log('CSV file successfully processed');
    console.log(res)
  });

