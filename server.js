const express = require("express");
const app = express();

app.use('/', express.static(__dirname));

app.listen(12000, () => {
    console.log(`Frontend web server listenting on port 12000. ${__dirname}`);
});
