import { BrowserRouter as Router, Routes, Route } from "react-router-dom";

import LandingPage from "./components/landing_page/LandingPage";
import React from "react";

export default function App() {
  return (
      <Router>
        <Routes>
          <Route path="/" element={<LandingPage />} />
        </Routes>
      </Router>
  )
}
