import { useState } from 'react'
import { Button } from './components/calculation-input/button'
import CalculationsPage from './pages/calculations/Calculations'
import { darkTheme, lightTheme } from './styles'
import './styles/style.css'


function App() {
  const [isDarkTheme, setIsDarkTheme] = useState(false)

  return (
    <div id="app" className={isDarkTheme ? darkTheme : lightTheme}>
      <CalculationsPage />
      <Button onClick={() => setIsDarkTheme(dark => !dark)} theme='secondary' >
        click
      </Button>
    </div>
  )
}

export default App
