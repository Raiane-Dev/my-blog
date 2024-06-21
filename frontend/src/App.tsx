import './App.less'
import { BrowserRouter } from "react-router-dom";
import { ConfigProvider, Layout } from 'antd';
import { Routes, Route, Outlet } from 'react-router-dom';
import { get } from './services/api_service';

import Home from './components/Home';
import Header from './components/Header';
import Footer from './components/Footer';
import Article from './components/Article';
import Login from './components/Login';
import FormPost from './components/CreatePost';
import ListPosts from './components/ListPosts';
import Sidebar from './components/Sidebar';
import { useEffect, useState } from 'react';

const App = () => {

  const Body = () => {
    return (
      <>
        <Layout className='layout-pattern'>
          <Layout>
            <Header />
            <Layout.Content className='content-pattern'>
              <Outlet />
            </Layout.Content>
          </Layout>
          <Footer />
        </Layout>
      </>
    )
  }

  const BodyPrivate = () => {
    return (
      <>
        <Layout className='layout-pattern'>
          <Layout>
            <Sidebar />
            <Layout.Content className='content-pattern'>
              <Outlet />
            </Layout.Content>
          </Layout>
          <Footer />
        </Layout>
      </>
    )
  }

  const [authenticated, setAuthenticated] = useState(false);

  async function isAuthenticated() {
    get("/check-auth")
      .then(() => setAuthenticated(true))
      .catch(() => setAuthenticated(false));
  }

  useEffect(() => {
    isAuthenticated();
  }, []);


  return (
    <ConfigProvider>
      <>
        <BrowserRouter>
          <Routes>
            <Route element={<Body />}>
              <Route index element={<Home />} />
              <Route path="/login" element={<Login />} />
              <Route path="/post/:post_id" element={<Article />} />
            </Route>

            {authenticated ?? (
              <Route path="/dashboard" element={<BodyPrivate />}>
                <Route path="/dashboard/home" element={<></>} />
                <Route path="/dashboard/create-post" element={<FormPost />} />
                <Route path="/dashboard/list-posts" element={<ListPosts />} />
              </Route>
            )}
          </Routes>
        </BrowserRouter>
      </>

    </ConfigProvider>
  );
};

export default App;