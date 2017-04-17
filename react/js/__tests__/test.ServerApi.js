import { getDiaryEntryDetails, getAllDiaryEntries } from '../requests.js'

/**
 * This file tests the API endpoints on the server
 * Since Rust does not have support for such testing, I thought I'd do it here
 * Needless to say, the Rust server needs to be running for these tests to pass
 */


// TODO: Load username/password; authenticate via endpoint and save the JWT Token

// it('Diary Entry 1 exists', done => {
//   getDiaryEntryDetails(1).then((diaryEntry) => {
//     expect(diaryEntry.id).toBe(1)
//     done()
//   }).catch(err => {
//     throw new Error(err)
//   })
// })

// it('Diary Entry with invalid ID should return an error', done => {
//   getDiaryEntryDetails(99999).then((diaryEntry) => {
//     throw new Error('Promise should have returned an error')
//   }).catch(err => {
//     expect(err.message).toBe('No entry with ID 99999 exists!')
//     done()
//   })
// })

it('Diary Details require authentication', done => {
  getDiaryEntryDetails(1).then((diaryEntry) => {
    throw new Error('Promise should have returned an error')
  }).catch(err => {
    expect(err.message).toBe("Unexpected error: Missing or Invalid JWT Token in the 'jwt-auth' header!")
    done()
  })
})

it('All Diary Details require authentication', done => {
  getAllDiaryEntries().then((diaryEntries) => {
    throw new Error('Promise should have returned an error')
  }).catch(err => {
    expect(err.message).toBe("Unexpected error: Missing or Invalid JWT Token in the 'jwt-auth' header!")
    done()
  })
})
