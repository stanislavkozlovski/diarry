import React from 'react'
import {submitNewDiary} from './requests.js'
import SweetAlert from 'sweetalert-react'

const NewDiary = React.createClass({
  //  TODO: Add textarea max text tracking
  //  TODO: Title and textarea validation such that "////" is not valid
  getInitialState () {
    // TODO: !!! Make it remember last body if you accidentally navigated off
    return {
      title: 'New Diary',
      body: '',
      timeouts: [],
      showAlert: false,
      alertTitle: '',
      alertDesc: ''
    }
  },

  handleTitleInput (event) {
    let newTitleText = event.target.value

    this.setState({title: newTitleText})
  },

  handleBodyInput (event) {
    this.setState({body: event.target.value})
  },

  handleNewDiarySubmit (event) {
    event.preventDefault()
    let diaryTitle = this.state.title
    let diaryBody = this.state.body
    if (diaryTitle.length <= 5) {
      this.state.showAlert = true
      this.setState({ showAlert: true, alertTitle: 'Title Too Short!', alertDesc: 'The title you entered is shorter than 5 characters!' })
    } else if (diaryBody.length <= 10) {
      this.setState({ showAlert: true, alertTitle: 'Content Too Short!', alertDesc: "Your entry's content is shorter than 10 characters!" })
    } else {
      submitNewDiary(diaryTitle, diaryBody).then(msg => {
        console.log(msg)
      })
    }
  },

  render () {
    const currDate = new Date()
    const shortMonth = currDate.toLocaleDateString('en-us', { month: 'short' })
    const day = currDate.getDate()
    const year = currDate.getFullYear()
    const currMinutes = currDate.getMinutes()
    const minutesString = (currMinutes < 10 ? '0' : '') + currMinutes
    const timeStr = `${currDate.getHours()}:${minutesString}`
    const dateString = `${shortMonth} ${day} ${year} - ${timeStr}`

    return (
      <section className='new-diary-post'>
        <SweetAlert
          type='error'
          show={this.state.showAlert}
          title={this.state.alertTitle}
          text={this.state.alertDesc}
          onConfirm={() => this.setState({ showAlert: false })}
        />
        <form className='new-diary-form' onSubmit={this.handleNewDiarySubmit} >
          <div className='new-diary-header'>
            <input value={this.state.title} className='new-diary-name' onChange={this.handleTitleInput} />
            <h3 className='new-diary-date'>{`${dateString}`}</h3>
          </div>

          <textarea className='new-diary-content' onChange={this.handleBodyInput} />
          <button className='new-diary-submit hvr-grow'>Submit</button>
        </form>
      </section>
    )
  }
})

export default NewDiary
