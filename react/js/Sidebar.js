import React from 'react'

const Sidebar = React.createClass({
  render () {
    return (
       <div id='sidebar'>
    <div id='logo'>
      <h1 className='mobileUI-site-name'>STRIPED</h1>
    </div>
    <nav id='nav' className='mobileUI-site-nav'>
      <ul>
        <li className='current_page_item'><a href='#'>Latest Post</a></li>
        <li><a href='#'>Archives</a></li>
        <li><a href='#'>About Me</a></li>
        <li><a href='#'>Contact Me</a></li>
      </ul>
    </nav>
    <section className='is-search is-first'>
      <form method='post' action='#'>
        <input type='text' className='text' name='search' placeholder='Search' />
      </form>
    </section>
    <section className='is-text-style1'>
      <div className='inner'>
        <p> <strong>Striped:</strong> A free and fully responsive HTML5 site template designed by AJ for HTML5 Up! </p>
      </div>
    </section>
    <section className='is-recent-posts'>
      <header>
        <h2>Recent Posts</h2>
      </header>
      <ul>
        <li><a href='#'>Nothing happened</a></li>
        <li><a href='#'>My Dearest Cthulhu</a></li>
        <li><a href='#'>The Meme Meme</a></li>
        <li><a href='#'>Now Full Cyborg</a></li>
        <li><a href='#'>Temporal Flux</a></li>
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
    <section className='is-calendar'>
      <div className='inner'>
        <table>
          <caption>
          February 2045
          </caption>
          <thead>
            <tr>
              <th scope='col' title='Monday'>M</th>
              <th scope='col' title='Tuesday'>T</th>
              <th scope='col' title='Wednesday'>W</th>
              <th scope='col' title='Thursday'>T</th>
              <th scope='col' title='Friday'>F</th>
              <th scope='col' title='Saturday'>S</th>
              <th scope='col' title='Sunday'>S</th>
            </tr>
          </thead>
          <tbody>
            <tr>
              <td colSpan='4' className='pad'><span>&nbsp;</span></td>
              <td><span>1</span></td>
              <td><span>2</span></td>
              <td><span>3</span></td>
            </tr>
            <tr>
              <td><span>4</span></td>
              <td><span>5</span></td>
              <td><a href='#'>6</a></td>
              <td><span>7</span></td>
              <td><span>8</span></td>
              <td><span>9</span></td>
              <td><a href='#'>10</a></td>
            </tr>
            <tr>
              <td><span>11</span></td>
              <td><span>12</span></td>
              <td><span>13</span></td>
              <td className='today'><a href='#'>14</a></td>
              <td><span>15</span></td>
              <td><span>16</span></td>
              <td><span>17</span></td>
            </tr>
            <tr>
              <td><span>18</span></td>
              <td><span>19</span></td>
              <td><span>20</span></td>
              <td><span>21</span></td>
              <td><span>22</span></td>
              <td><a href='#'>23</a></td>
              <td><span>24</span></td>
            </tr>
            <tr>
              <td><a href='#'>25</a></td>
              <td><span>26</span></td>
              <td><span>27</span></td>
              <td><span>28</span></td>
              <td className='pad' colSpan='3'><span>&nbsp;</span></td>
            </tr>
          </tbody>
        </table>
      </div>
    </section>
    </div>
    )
  }
})

export default Sidebar
