import React from "react";
import { BrowserRouter as Router, Routes, Route } from "react-router-dom";

import DmPage from "./components/DmPage/DmPage";
import Dashboard from "./components/Dashboard/Dashboard";
import LandingPage   from "./components/LandingPage/LandingPage";
import LoginPage     from "./components/LoginRegisterPage/LoginPage";

export default function App() {
  return (
      <Router>
        <Routes>
          <Route path="/"           element={<LandingPage />} />
          <Route path="/login"      element={<LoginPage />} />
          <Route path="/register"   element={<LoginPage />} />
          <Route path="/dashboard"  element={<Dashboard />} />
          <Route path="/dm/:id"     element={<DmPage />} />
        </Routes>
      </Router>
  )
}
