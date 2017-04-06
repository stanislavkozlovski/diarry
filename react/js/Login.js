import React from 'react'

import '../public/login-style.css'

const Login = React.createClass({
  handleEmailInput (event) {
    this.setState({email: event.target.value})
  },

  handlePassInput (event) {
    this.setState({pass: event.target.value})
  },

  handleLogIn (event) {
    event.preventDefault()

    let email = this.state.email
    let password = this.state.password

    // TODO: POST credentials to server and await response
  },

  render () {
    return (
      <div className='login'>
        <div id='login'>
          <form onSubmit={this.handleLogIn}>
            <fieldset className='clearfix'>
              <p>
                <span className='fontawesome-user' />
                <input type='text' name='email' placeholder='E-mail' required onChange={this.handleEmailInput} /></p>
              <p>
                <span className='fontawesome-lock' />
                <input type='password' name='pass' placeholder='Password' required onChange={this.handlePassInput} /></p>
              <p><input type='submit' value='Sign In' /></p>
            </fieldset>
          </form>
          <p>Not a member? Sorry <span className='fontawesome-arrow-right' /></p>
        </div>
      </div>
    )
  }
})

export default Login
