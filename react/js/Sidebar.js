import React from 'react'
import { Link } from 'react-router'
import {getLastFiveDiaryEntryMetaData} from './requests.js'

import InfiniteCalendar from 'react-infinite-calendar'
import 'react-infinite-calendar/styles.css'

const Sidebar = React.createClass({
  getInitialState () {
    return {
      recentPosts: [],
      showCalendar: false
    }
  },

  componentDidMount () {
    // load the last 5 posts
    getLastFiveDiaryEntryMetaData().then(recentPosts => {
      this.setState({recentPosts})
    }).catch(err => {
      console.log(err)
    })
  },

  handleShowCalendar () {
    // TODO: Calendar logic to redirect on date selection and highlight days with posts on them
    // also to close on click outside
    let today = new Date()
    let minDate = new Date(1996, 9, 17)
    let maxDate = new Date(today.getFullYear(), today.getMonth() + 1, today.getDate())
    this.setState({showCalendar: true, minDate, maxDate})
  },

  render () {
    let calendar = undefined
    if (this.state.showCalendar) {
      calendar = (
        <InfiniteCalendar
          width={500}
          height={400}
          selected={new Date()}
          minDate={this.state.minDate}
          maxDate={this.state.maxDate}
          min={this.state.minDate}
          max={this.state.maxDate}
        />
      )
    }
    // <li><a href='#' onClick={this.handleShowCalendar}>Search by date</a></li>
    // <p> <strong>Diarry:</strong> A personal diary written in Rocket.rs and React.js! </p>
    return (
      <aside className='left-sidebar'>
        <div className='new-post-container hvr-shutter-out-horizontal'>
          <Link to={'/entry/new'}><button className='new-post'>New Post</button></Link>
        </div>

        <div className='search-box'>
          <div className='icon-box'>
            <span><i className='fa fa-search' aria-hidden='true' style={{color: 'white'}} /></span>
          </div>
          <input />
        </div>
        <div className='latest-posts'>
          <p>Latest Posts</p>
          {this.state.recentPosts.map(post => {
            return <div className='latest-post' key={post.url} ><Link to={post.url}>{post.title}</Link></div>
          })}
        </div>

        <div className='latest-comments'>
          <p>Latest Comments</p>
          <div className='latest-comment'>
            <p>Hello</p>
          </div>
          <div className='latest-comment'>
            <p>Hello</p>
          </div>
          <div className='latest-comment'>
            <p>Hello</p>
          </div>
          <div className='latest-comment'>
            <p>Hello</p>
          </div>
        </div>

      </aside>

    )
  }
})

export default Sidebar
