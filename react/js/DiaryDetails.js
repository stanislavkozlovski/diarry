import React from 'react'
import axios from 'axios'
import DiaryEntry from './DiaryEntry.js'
const { shape, string } = React.PropTypes
// maybe turn into a stateless component and have a higher comp do AJAX :)
const DiaryDetails = React.createClass({
  propTypes: {
    params: shape({
      id: string
    })
  },

  componentWillReceiveProps (nextProps) {
    // TODO: Maybe add some sort of animation

    /* query the backend for details for a specific DiaryEntry and show it to the user */
    axios.get(`http://localhost:8000/api/entries/${nextProps.params.id}`)
    .then(resp => {
      this.setState(resp.data)
    }).catch(err => {
      if (err.response) {
        if (err.response.status === 404) {
          this.setState({error: `No entry with ID ${nextProps.params.id} exists!`})
        } else {
          console.log(`Unexpected error: ${err}`)
        }
      } else {
        console.log(`Unexpected error: ${err}`)
      }
    })
  },

  componentDidMount () {
    /* query the backend for details for a specific DiaryEntry and show it to the user */
    axios.get(`http://localhost:8000/api/entries/${this.props.params.id}`)
    .then(resp => {
      this.setState(resp.data)
    }).catch(err => {
      if (err.response) {
        if (err.response.status === 404) {
          this.setState({error: `No entry with ID ${this.props.params.id} exists!`})
        } else {
          console.log(`Unexpected error: ${err}`)
        }
      } else {
        console.log(`Unexpected error: ${err}`)
      }
    })
  },

  render () {
    if (this.state && this.state.body) {
      return (
        <DiaryEntry {...this.state} />
      )
    } else if (this.state && this.state.error) {
      return <h1 className='alert alert-danger'> {this.state.error} </h1>
    } else {
      return <h1> Loading </h1>
    }
  }
})

export default DiaryDetails
