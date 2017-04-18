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
        <div>
          {
            this.state.diaryEntries.map(entry => {
              return <DiaryEntry {...entry} key={entry.id} />
            })
          }
        </div>
      )
    } else {
      return (
        <h1>Loading Entries</h1>
      )
    }
  }
})

export default LandingPage
