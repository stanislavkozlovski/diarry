import React from 'react'
import { render } from 'react-dom'
import { BrowserRouter, Match } from 'react-router'


import '../public/css/5grid/core-desktop.css'
import '../public/css/5grid/core-1200px.css'
import '../public/css/5grid/core-noscript.css'
import '../public/css/5grid/core.css'
import '../public/css/style.css'
import '../public/css/style-desktop.css'
import '../public/css/style-1200px.css'

const App = React.createClass({
  render: function () {
    return (
      <BrowserRouter>
        <div className='app' />
      </BrowserRouter>
    )
  }
})

render(
  (
  <div>
    <App />
  </div>
  ), document.getElementById('app')
)
