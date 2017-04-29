// Various helper methods

/**
 * Return an object which holds string formatted date properties, like minutes, hours, days and etc
 * @param {String} date - a string Date object, representing days, months years. Date() should recognize it.
 * @param {String} time - a string Time object, representing hours minutes and seconds
 * @return {Object} - An object which has a year, day (with padded zeroes i.e 01), shortMonth (Jan), time (with padded zeroes, i.e 01:01), monthNum (01) and an all out dateString - Jan 10 2017 - 14:02
 */
function getDateObject (wantedDate, wantedTime) {
  let dateStr = wantedDate || ''
  let timeStr = wantedTime || ''
  let dateObj = null

  if (wantedDate === undefined && wantedTime === undefined) {
    dateObj = new Date()
  } else {
    dateObj = new Date(dateStr + ' ' + timeStr)
  }

  let shortMonth = dateObj.toLocaleDateString('en-us', { month: 'short' })
  let date = dateObj.getDate()
  let dayStr = date < 10 ? '0' + date : date.toString()
  let year = dateObj.getFullYear()
  let hour = dateObj.getHours()
  let hourStr = hour < 10 ? '0' + hour : hour.toString()
  let minutes = dateObj.getMinutes()
  let minutesStr = minutes < 10 ? '0' + minutes : minutes.toString()

  let timeDisplay = `${hourStr}:${minutesStr}`

  return {
    dateObj: dateObj,
    year: year,
    day: dayStr,
    timeDisplay: timeDisplay,
    shortMonth: shortMonth,
    dateString: `${shortMonth} ${dayStr} ${year} - ${timeDisplay}`
  }
}

/**
 * A comparator function which takes two objects which preferably have a dateObj: Date() field in them
 * It is meant to be used as a comparator function in a sort() function
 * @param {Object} a
 * @param {Object} b
 */
const sortByDateDescending = (a, b) => { return b.dateObj - a.dateObj }

export {getDateObject, sortByDateDescending}
