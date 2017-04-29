import React from 'react'

import DiaryEntry from './DiaryEntry.js'
import {getAllDiaryEntries} from './requests.js'

const LandingPage = React.createClass({
  componentDidMount () {
    getAllDiaryEntries().then(diaryEntries => {
      this.setState({diaryEntries: diaryEntries})
    }).catch(err => {
      console.log(err)
    })
  },

  render () {
    if (this.state && this.state.diaryEntries) {
      return (
        <section className='diary-entries'>
          {
            this.state.diaryEntries.map(entry => {
              return <DiaryEntry {...entry} key={entry.id} isMetaInfo />
            })
          }
        </section>
      )
    } else {
      return (
        <h1>Loading Entries</h1>
      )
    }
  }
})

export default LandingPage
