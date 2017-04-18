/**
 * Reads the email and password from the .env file
 */
import path from 'path'
import axios from 'axios'


const dotEnvPath = path.join(__dirname, '../../rust/.env')
require('dotenv').config({path: dotEnvPath})

const userEmail = process.env.EMAIL
const userPassword = process.env.PASSWORD

/**
 * Queries the server to get the JWT Token and sets it as a axios default header
 */
function setJWTTokenHeader () {
  return new Promise((resolve, reject) => {
    axios.post('http://localhost:8000/api/authenticate', { email: userEmail, password: userPassword }).then(resp => {
      // successful login, save the token and redirect to the homepage
      let authToken = resp.data
      axios.defaults.headers.common['jwt-auth'] = authToken
      resolve(authToken)
    }).catch(err => {
      reject(err)
    })
  })
}

/**
 * Removes the axios jwt-auth header
 */
function removeJWTTokenHeader () {
  delete axios.defaults.headers.common['jwt-auth']
}

export {removeJWTTokenHeader, setJWTTokenHeader}
