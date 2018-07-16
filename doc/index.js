const express = require('express')
const pathToSwaggerUi = require('swagger-ui-dist').absolutePath()

const app = express()

app.use('/assets', express.static('assets'))
app.use(express.static(pathToSwaggerUi))

app.listen(3000)
