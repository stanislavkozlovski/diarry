import React from 'react'
import axios from 'axios'

const { shape, string } = React.PropTypes
// maybe turn into a stateless component and have a higher comp do AJAX :)
const DiaryEntry = React.createClass({
  propTypes: {
    params: shape({
      id: string
    })
  },
  componentDidMount () {
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
        <article className='is-post is-post-excerpt'>
          <header>
            <h2><a href='#'>{this.state.title}</a></h2>
          </header>
          <div className='info'>
            <span className='date'>
              <span className='month'>
                {this.state.date}
              </span>
              <span className='day'>
                {this.state.hour}
              </span>
              <span className='year'>
                , {this.state.date}
              </span>
            </span>

          </div>
          <a href='#' className='image image-full'><img src='images/n33-robot-invader.jpg' alt='' /></a>
          <p>{this.state.body}</p>
        </article>
      )
    } else if (this.state && this.state.error) {
      return <h1 className='alert alert-danger'> {this.state.error} </h1>
    } else {
      return <h1> Loading </h1>
    }
  }
})

export default DiaryEntry
