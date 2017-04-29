import React from 'react'

import DiaryEntry from './DiaryEntry.js'
import {getAllDiaryEntries} from './requests.js'
import {PAGE_HEADER_CSS_HEIGHT} from './constants.js'

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
      let diaryWrapperStyle = {
        marginRight: '-20px',
        overflowY: 'scroll',
        height: (window.innerHeight - PAGE_HEADER_CSS_HEIGHT) + 'px'
      }

      return (
        <section className='diary-entries'>
          <section className='entry-wrapper' style={diaryWrapperStyle}>
            {
              this.state.diaryEntries.map(entry => {
                return <DiaryEntry {...entry} key={entry.id} isMetaInfo />
              })
            }
          </section>
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
