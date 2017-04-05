import React from 'react'
import axios from 'axios'

import DiaryEntry from './DiaryEntry.js'

const LandingPage = React.createClass({
  componentDidMount () {
    axios.get(`http://localhost:8000/api/entries/all`).then((resp) => {
      this.setState({diaryEntries: resp.data})
    }).catch(err => {
      console.log(`Unexpected error: ${err}`)
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
