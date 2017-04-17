import axios from 'axios'

/**
 * Queries the backend for details for a specific DiaryEntry
 * @param {Number} diaryEntryId - The ID of the DiaryEntry
 * @return {Object} - the resulting DiaryEntry or an Error
 */

function getDiaryEntryDetails (diaryEntryId) {
  return axios.get(`http://localhost:8000/api/entries/${diaryEntryId}`)
  .then(resp => {
    console.log('Found error?')
    return resp.data
  }).catch(err => {
    if (err.response) {
      if (err.response.status === 404) {
        throw new Error(`No entry with ID ${diaryEntryId} exists!`)
      } else {
        throw new Error(`Unexpected error: ${err}`)
      }
    } else {
      throw new Error(`Unexpected error: ${err}`)
    }
  })
}

export { getDiaryEntryDetails }
