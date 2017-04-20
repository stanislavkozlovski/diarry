import React from 'react'
const { string } = React.PropTypes

const DiaryEntry = (props) => {
  // parse the date to a Date object
  const date = new Date(props.creation_date + ' ' + props.creation_time)
  const shortMonth = date.toLocaleString('en-us', { month: 'short' })
  const year = date.getFullYear()
  const timeStr = `${date.getHours()}:${date.getMinutes()}`
  return (
    <article className='is-post is-post-excerpt'>
      <header>
        <h2><a href='#'>{props.title}</a></h2>
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
      <p>{props.body}</p>
    </article>
  )
}

DiaryEntry.propTypes = {
  creation_date: string,
  creation_time: string,
  title: string,
  body: string
}

export default DiaryEntry
