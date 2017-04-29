import React from 'react'
import { render } from 'react-dom'
import { HashRouter, Match } from 'react-router'
import axios from 'axios'

import DiaryDetails from './DiaryDetails.js'
import LandingPage from './Landing.js'
import NewDiary from './NewDiary.js'
import Sidebar from './Sidebar.js'
import Auth from './auth.js'
// import '../public/css/third-party/core-desktop.css'
// import '../public/css/third-party/core-1200px.css'
// import '../public/css/third-party/core-noscript.css'
// import '../public/css/third-party/core.css'
// import '../public/css/style-desktop.css'
// import '../public/css/style-1200px.css'
import '../public/css/style.css'
import Login from './Login.js'

// set the JWT token header
if (window.localStorage && localStorage['jwt-auth']) {
  axios.defaults.headers.common['jwt-auth'] = localStorage['jwt-auth']
}

const App = React.createClass({
  render: function () {
    if (!Auth.isUserAuthenticated()) {
      return <Login />
    }
    /* <div className='app'>
        <div id='wrapper'>
          <div id='content' className='mobileUI-main-content'>
            <div id='content-inner'>
              <Match exactly pattern='/entry/new' component={NewDiary} />
              <Match exactly pattern='/entry/:id' component={DiaryDetails} />
              <Match exactly pattern='/' component={LandingPage} />
            </div>
          </div>
        </div>
      </div> */
    return (
      <div>
        <header className='title-header'>
          <div className='title-box'>
            <h1 className='title'>Diarry</h1>
          </div>
        </header>
        <main>
          <Sidebar />
          <Match exactly pattern='/entry/new' component={NewDiary} />
          <Match exactly pattern='/entry/:id' component={DiaryDetails} />
          <Match exactly pattern='/' component={LandingPage} />
        </main>
      </div>
    )
  }
})


render(
  (
  <div>
    <HashRouter>
      <App />
    </HashRouter>
  </div>
  ), document.getElementById('app')
)
