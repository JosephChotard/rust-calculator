import { ColorModeProvider, ColorModeToggle } from './components/colour-mode/ColourMode'
import CalculationsPage from './pages/calculations/Calculations'
import './styles/style.css'


function App() {


  return (
    <ColorModeProvider>
      <ColorModeToggle />
      <CalculationsPage />
    </ColorModeProvider>
  )
}

export default App
