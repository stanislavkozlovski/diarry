import axios from 'axios'

/**
 * Queries the backend for details for a specific DiaryEntry
 * @param {Number} diaryEntryId - The ID of the DiaryEntry
 * @return {Object} - the resulting DiaryEntry or an Error
 */

function getDiaryEntryDetails (diaryEntryId) {
  return axios.get(`http://localhost:8000/api/entries/${diaryEntryId}`)
  .then(resp => {
    return resp.data
  }).catch(err => {
    if (err.response) {
      if (err.response.status === 404) {
        throw new Error(`No entry with ID ${diaryEntryId} exists!`)
      } else {
        if (err.response.data !== undefined && err.response.data.error_message !== undefined) {
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
  return axios.get(`http://localhost:8000/api/entries/all`).then((resp) => {
    return resp.data
  }).catch(err => {
    if (err.response && err.response.status === 401 && err.response.data.error_message) {
      throw new Error(`Unexpected error: ${err.response.data.error_message}`)
    }
    throw new Error(`Unexpected error: ${err}`)
  })
}

function submitNewDiary (title, body) {
  return axios.post('http://localhost:8000/api/entries/new', {
    title,
    body
  }).then(resp => {
    return resp
  }).catch(err => {
    if (err.response && err.response.status === 401 && err.response.data.error_message) {
      throw new Error(`Unexpected error: ${err.response.data.error_message}`)
    }
    return `Unexpected error: ${err}`
  })
}


export { getDiaryEntryDetails, getAllDiaryEntries, submitNewDiary }

/* TODO: Authentication */
