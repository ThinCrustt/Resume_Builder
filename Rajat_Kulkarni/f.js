
var express = require('express');
var exphbs  = require('express-handlebars');

var app = express();

app.engine('handlebars', exphbs());
app.set('view engine', 'handlebars');

Handlebars.registerHelper('loud', function (aString) {
    return aString.toUpperCase()
})


