import React from 'react'
import DiaryEntry from './DiaryEntry.js'
import {getDiaryEntryDetails} from './requests.js'
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
    getDiaryEntryDetails(nextProps.params.id).then(data => {
      this.setState(data)
    }).catch(err => {
      this.setState({error: err.message})
    })
  },

  componentDidMount () {
    /* query the backend for details for a specific DiaryEntry and show it to the user */
    getDiaryEntryDetails(this.props.params.id).then(data => {
      this.setState(data)
    }).catch(err => {
      this.setState({error: err.message})
    })
  },

  reload () {
    getDiaryEntryDetails(this.props.params.id).then(data => {
      this.setState(data)
    }).catch(err => {
      this.setState({error: err.message})
    })
  },

  render () {
    if (this.state && this.state.body) {
      return (
        <DiaryEntry {...this.state} reload={this.reload} />
      )
    } else if (this.state && this.state.error) {
      return <h1 className='alert alert-danger'> {this.state.error} </h1>
    } else {
      return <h1> Loading </h1>
    }
  }
})

export default DiaryDetails
