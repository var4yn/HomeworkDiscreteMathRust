import { Routes, Route, BrowserRouter, useLocation } from "react-router-dom";
import { AnimatePresence } from "framer-motion";
import MenuPage from "./pages/MenuPage";
import TaskPage from "./pages/TaskPage";

function App() {
  const location = useLocation();

  return (
    <AnimatePresence mode="wait">
      <Routes location={location} key={location.pathname}>
        <Route index path="/" element={<MenuPage/>} />
        <Route path="/:id" element={<TaskPage/>}/>
      </Routes>
    </AnimatePresence>    
  )
}

export default App;
