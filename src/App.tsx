import { ColorModeProvider } from './components/colour-mode/ColourMode'
import CalculationsPage from './pages/calculations/Calculations'
import './styles/style.css'


function App() {


  return (
    <ColorModeProvider>
      <CalculationsPage />
    </ColorModeProvider>
  )
}

export default App
