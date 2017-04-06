import React from 'react'
import axios from 'axios'
import SweetAlert from 'sweetalert-react'

const NewDiary = React.createClass({
  //  TODO: Add textarea max text tracking
  //  TODO: Title and textarea validation such that "////" is not valid
  getInitialState () {
    // TODO: !!! Make it remember last body if you accidentally navigated off
    return {
      title: '',
      body: '',
      timeouts: [],
      badTitle: false,
      showAlert: false,
      alertTitle: '',
      alertDesc: ''
    }
  },

  handleTitleInput (event) {
    // Set a timeout that makes the input box glow red if its too short
    let newTitleText = event.target.value
    if (newTitleText.length <= 5) {
      this.state.timeouts.push(setTimeout(() => {
        if (newTitleText.length <= 5 && !this.state.showAlert) {
          this.setState({badTitle: true})
        }
      }, 100))
    } else {
      // clear timeouts
      for (var i = 0; i < this.state.timeouts.length; i++) {
        clearTimeout(this.state.timeouts[i])
      }
      this.setState({badTitle: false})
    }

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
      axios.post('http://localhost:8000/api/entries/new', {
        title: this.state.title,
        body: this.state.body
      }).then(resp => {
        console.log(resp)
      }).catch(err => {
        console.log(`Error: ${err}`)
      })
    }
  },

  render () {
    let titleClass = this.state.badTitle ? 'new-entry-title new-entry-bad-title' : 'new-entry-title'
    return (
      <section className='box post post-excerp'>
        <SweetAlert
          type='error'
          show={this.state.showAlert}
          title={this.state.alertTitle}
          text={this.state.alertDesc}
          onConfirm={() => this.setState({ showAlert: false })}
        />
        <form onSubmit={this.handleNewDiarySubmit}>
          <div className='new-entry-header'>
            <h1 className='new-entry'>New Entry</h1>
          </div>
          <input name='title' placeholder='The title of your entry' className={titleClass} onChange={this.handleTitleInput} />
          <textarea name='body' className='entry-body' onChange={this.handleBodyInput} />
        </form>
      </section>
    )
  }
})

export default NewDiary
