import './App.css'
import { OverpassApiService } from './services/overpass-api'

function App() {
  
  const overpassApiService = new OverpassApiService()

  return (
    <>
      <div className="card">
        <button onClick={() => overpassApiService.makeSimpleRequest()}>
          Make Request To OverPassAPI
        </button>
      </div>
     
    </>
  )
}

export default App
