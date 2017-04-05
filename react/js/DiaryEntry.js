import React from 'react'

const DiaryEntry = (props) => {
  return (
    <article className='is-post is-post-excerpt'>
      <header>
        <h2><a href='#'>{props.title}</a></h2>
      </header>
      <div className='info'>
        <span className='date'>
          <span className='month'>
            {props.date}
          </span>
          <span className='day'>
            {props.hour}
          </span>
          <span className='year'>
            , {props.date}
          </span>
        </span>
      </div>
      <p>{props.body}</p>
    </article>
  )
}

export default DiaryEntry
