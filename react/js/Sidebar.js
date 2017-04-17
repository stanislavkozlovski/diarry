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
    return (
      <div id='sidebar'>
        <div id='logo'>
          <h1 className='mobileUI-site-name'>Username's Diary</h1>
        </div>
        <Link to='/entry/new'>
          <div id='newEntry'>
            <h1 className='mobileUI-site-name'>New Entry</h1>
          </div>
        </Link>
        <nav id='nav' className='mobileUI-site-nav'>
          <ul>
            <li className='current_page_item'><a href='#'>Latest Post</a></li>
            <li><a href='#'>Archives</a></li>
            <li><a href='#'>About Me</a></li>
            <li><a href='#' onClick={this.handleShowCalendar}>Search by date</a></li>
          </ul>
          {calendar}
        </nav>
        <section className='is-search is-first'>
          <form method='post' action='#'>
            <input type='text' className='text' name='search' placeholder='Search' disabled />
          </form>
        </section>
        <section className='is-text-style1'>
          <div className='inner'>
            <p> <strong>Diarry:</strong> A personal diary written in Rocket.rs and React.js! </p>
          </div>
        </section>
        <section className='is-recent-posts'>
          <header>
            <h2>Recent Posts</h2>
          </header>
          <ul>
            {this.state.recentPosts.map(post => {
              return <li key={post.url} ><Link to={post.url}>{post.title}</Link></li>
            })}
          </ul>
        </section>
        <section className='is-recent-comments'>
          <header>
            <h2>Recent Comments</h2>
          </header>
          <ul>
            <li>case on <a href='#'>Now Full Cyborg</a></li>
            <li>molly on <a href='#'>Untitled Post</a></li>
            <li>case on <a href='#'>Temporal Flux</a></li>
          </ul>
        </section>
      </div>
    )
  }
})

export default Sidebar
