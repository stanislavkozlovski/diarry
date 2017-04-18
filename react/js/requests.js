import Auth from './auth.js'
import axios from 'axios'

/**
 * Fills a config with headers of the required JWT authentication
 * @return {Object} - A config to be used on an axios request
 */
function getAxiosConfig () {
  return {headers: {'jwt-auth': Auth.getToken() || ''}}
}

/**
 * Queries the backend for details for a specific DiaryEntry
 * @param {Number} diaryEntryId - The ID of the DiaryEntry
 * @return {Object} - the resulting DiaryEntry or an Error
 */

function getDiaryEntryDetails (diaryEntryId) {
  return axios.get(`http://localhost:8000/api/entries/${diaryEntryId}`, {}, getAxiosConfig())
  .then(resp => {
    return resp.data
  }).catch(err => {
    if (err.response) {
      if (err.response.status === 404) {
        throw new Error(`No entry with ID ${diaryEntryId} exists!`)
      } else {
        if (err.response.data !== undefined && err.response.data.error_message !== undefined) {
          // JWT Token is non-existant or expired
          Auth.deauthenticateUser()
          throw new Error(`Unexpected error: ${err.response.data.error_message}`)
        }
        throw new Error(`Unexpected error: ${err}`)
      }
    } else {
      throw new Error(`Unexpected error: ${err}`)
    }
  })
}

/**
 * Queries the server for all the Diary Entries ever created
 * @return {Promise} - an axios GET query to /api/entries/all, which should return all the diary entries
 */
function getAllDiaryEntries () {
  return axios.get(`http://localhost:8000/api/entries/all`, {}, getAxiosConfig()).then((resp) => {
    return resp.data
  }).catch(err => {
    if (err.response && err.response.status === 401 && err.response.data.error_message) {
      // JWT Token is non-existant or expired
      Auth.deauthenticateUser()
      throw new Error(`Unexpected error: ${err.response.data.error_message}`)
    }
    throw new Error(`Unexpected error: ${err}`)
  })
}

/**
 * Queries the server for the last five diary entries, fetching only meta data for them
 * @return {Promise} - an axios POST query to /api/entries/last_five, which should return the meta data about the last five entries
 */
function getLastFiveDiaryEntryMetaData () {
  return axios.get('http://localhost:8000/api/entries/last_five', {}, getAxiosConfig())
  .then(resp => {
    return resp.data
  }).catch(err => {
    if (err.response && err.response.status === 401 && err.response.data.error_message) {
      // JWT Token is non-existant or expired
      Auth.deauthenticateUser()
      throw new Error(`Unexpected error: ${err.response.data.error_message}`)
    }
    throw new Error(`Unexpected error: ${err}`)
  })
}

/**
 * Submits a POST request to the server for creating a new DiaryEntry
 * @param {String} title - the title of the diary entry
 * @param {String} body - the body of the diary entry
 */
function submitNewDiary (title, body) {
  return axios.post('http://localhost:8000/api/entries/new', {
    title,
    body
  }, getAxiosConfig()).then(resp => {
    return resp
  }).catch(err => {
    if (err.response && err.response.status === 401 && err.response.data.error_message) {
      // JWT Token is non-existant or expired
      Auth.deauthenticateUser()
      throw new Error(`Unexpected error: ${err.response.data.error_message}`)
    }
    return `Unexpected error: ${err}`
  })
}


export { getDiaryEntryDetails, getLastFiveDiaryEntryMetaData, getAllDiaryEntries, submitNewDiary }

/* TODO: Authentication */
