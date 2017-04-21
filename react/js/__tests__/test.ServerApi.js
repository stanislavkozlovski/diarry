import { getDiaryEntryDetails, getLastFiveDiaryEntryMetaData, getAllDiaryEntries, submitNewDiary, submitNewComment } from '../requests.js'
import {removeJWTTokenHeader, setJWTTokenHeader} from '../testBase.js'

/**
 * This file tests the API endpoints on the server
 * Since Rust does not have support for such testing, I thought I'd do it here
 * Needless to say, the Rust server needs to be running for these tests to pass
 */


it('Diary Entry 1 exists', done => {
  setJWTTokenHeader().then(() => {
    getDiaryEntryDetails(1).then((diaryEntry) => {
      expect(diaryEntry.id).toBe(1)
      done()
    }).catch(err => {
      throw new Error(err)
    })
  })
})


it('All Diaries returns array', done => {
  setJWTTokenHeader().then(() => {
    getAllDiaryEntries().then((diaryEntries) => {
      expect(Array.isArray(diaryEntries)).toBe(true)
      expect(diaryEntries.length > 1).toBe(true)
      done()
    }).catch(err => {
      throw new Error('Promise should not have returned an error' + err)
    })
  })
})

it('Diary Entry with invalid ID should return an error', done => {
  getDiaryEntryDetails(99999).then((diaryEntry) => {
    throw new Error('Promise should have returned an error')
  }).catch(err => {
    expect(err.message).toBe('No entry with ID 99999 exists!')
    done()
  })
})

it('Diary Details require authentication', done => {
  removeJWTTokenHeader()
  getDiaryEntryDetails(1).then((diaryEntry) => {
    throw new Error('Promise should have returned an error')
  }).catch(err => {
    expect(err.message).toBe("Unexpected error: Missing or Invalid JWT Token in the 'jwt-auth' header!")
    done()
  })
})

it('All Diary Details require authentication', done => {
  removeJWTTokenHeader()
  getAllDiaryEntries().then((diaryEntries) => {
    throw new Error('Promise should have returned an error')
  }).catch(err => {
    expect(err.message).toBe("Unexpected error: Missing or Invalid JWT Token in the 'jwt-auth' header!")
    done()
  })
})

it('Submit New Diary route requires authentication', done => {
  removeJWTTokenHeader()
  
  submitNewDiary('Test Title ', 'Test Body Test Body').then(() => {
    throw new Error('Promise should have returned an error')
  }).catch(err => {
    expect(err.message).toBe("Unexpected error: Missing or Invalid JWT Token in the 'jwt-auth' header!")
    done()
  })
})

it('Submit New Comment route requires authentication', done => {
  removeJWTTokenHeader()
  
  submitNewComment(1, 'What the f is up my man?').then(() => {
    throw new Error('Promise should have returned an error')
  }).catch(err => {
    expect(err.message).toBe("Unexpected error: Missing or Invalid JWT Token in the 'jwt-auth' header!")
    done()
  })
})

it('Last Five Diaries requires authentication', done => {
  removeJWTTokenHeader()
  getLastFiveDiaryEntryMetaData().then(() => {
    throw new Error('Promise should have returned an error')
  }).catch(err => {
    expect(err.message).toBe("Unexpected error: Missing or Invalid JWT Token in the 'jwt-auth' header!")
    done()
  })
})

