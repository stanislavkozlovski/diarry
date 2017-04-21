import React from 'react'
import {submitNewComment} from './requests.js'
const { array, func, string, number } = React.PropTypes

const DiaryEntry = React.createClass({

  propTypes: {
    id: number,
    creation_date: string,
    creation_time: string,
    title: string,
    body: string,
    comments: array,
    reload: func
  },

  getInitialState () {
    return {
      commentBody: ''
    }
  },

  handleCommentInput (event) {
    this.setState({commentBody: event.target.value})
  },

  handleNewCommentSubmit (event) {
    event.preventDefault()
    // TODO: comment validation
    let commentBody = this.state.commentBody
    submitNewComment(this.props.id, commentBody).then(msg => {
      // reload the parent DiaryDetails
      this.props.reload()
    })
  },

  render () {
    // parse the date to a Date object
    const date = new Date(this.props.creation_date + ' ' + this.props.creation_time)
    const shortMonth = date.toLocaleString('en-us', { month: 'short' })
    const year = date.getFullYear()
    const timeStr = `${date.getHours()}:${date.getMinutes()}`

    if (this.props.isMetaInfo) {
      return (
        <article className='is-post is-post-excerpt'>
          <header>
            <h2><a href='#'>{this.props.title}</a></h2>
          </header>
          <div className='info'>
            <span className='date'>
              <span className='month'>
                {shortMonth}
              </span>
              <span className='day'>
                {date.getDate()}
              </span>
              <span className='year'>
                {year}
              </span>
            </span>
            <span className='time'>
              <span>{timeStr}</span>
            </span>
          </div>
          <p>{this.props.body}</p>
        </article>
      )
    }
    // if we're here, this DiaryEntry must be called from DiaryDetails
    // map the user friendly dates to the article comments
    this.props.comments.map((comment) => {
      const commentDate = new Date(comment.creation_date + ' ' + comment.creation_time)
      comment.shortMonth = date.toLocaleDateString('en-us', { month: 'short' })
      comment.day = commentDate.getDate()
      comment.year = commentDate.getFullYear()
      comment.timeStr = `${commentDate.getHours()}:${commentDate.getMinutes()}`
      comment.dateString = `${comment.shortMonth} ${comment.day} ${comment.year} - ${comment.timeStr}`
    })
    return (
      <article className='is-post is-post-excerpt'>
        <header>
          <h2><a href='#'>{this.props.title}</a></h2>
        </header>
        <div className='info'>
          <span className='date'>
            <span className='month'>
              {shortMonth}
            </span>
            <span className='day'>
              {date.getDate()}
            </span>
            <span className='year'>
              {year}
            </span>
          </span>
          <span className='time'>
            <span>{timeStr}</span>
          </span>
        </div>
        <p>{this.props.body}</p>

        <div className='article-comments'>
          {this.props.comments.map((comment) => {
            return (
              <div className='article-comment'>
                <div className='comment-header'>
                  <p>{comment.dateString}</p>
                </div>
                <div className='comment-content'>
                  <p>{comment.body}</p>
                </div>
              </div>
            )
          })}
          <form onSubmit={this.handleNewCommentSubmit}>
            <div className='article-comment'>
              <textarea name='commentBody' className='new-comment' onChange={this.handleCommentInput} />
              <button type='submit'> COMMENT </button>
            </div>
          </form>
        </div>
      </article>
    )
  }
})


export default DiaryEntry
