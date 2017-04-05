import React from 'react'
import { render } from 'react-dom'
import { HashRouter, Match } from 'react-router'

import DiaryDetails from './DiaryDetails.js'
import LandingPage from './Landing.js'

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
      <HashRouter>
        <div className='app'>
          <div id='content' className='mobileUI-main-content'>
            <div id='content-inner'>
              <Match exactly pattern='/entry/:id' component={DiaryDetails} />
              <Match exactly pattern='/' component={LandingPage} />
            </div>
          </div>
        </div>
      </HashRouter>
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
