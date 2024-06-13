import { FC } from 'react';
import { BrowserRouter } from "react-router-dom";
import { ConfigProvider, Layout } from 'antd';
import { Routes, Route, Outlet } from 'react-router-dom';
import pt_br from 'antd/locale/pt_BR'
import './App.less'
import Home from './components/Home';
import Header from './components/Header';
import Footer from './components/Footer';
import Article from './components/Article';
import Cookies from 'universal-cookie';
import Login from './components/Login';
import FormPost from './components/CreatePost';
import ListPosts from './components/ListPosts';
import Sidebar from './components/Sidebar';

const cookies = new Cookies();

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

const App: FC = () => {
  const isAuthenticated = cookies.get("x-access-token") != undefined ? true : false;

  return (
    <>
      <ConfigProvider locale={pt_br}>
        <>
          <BrowserRouter>
            <Routes>
              <Route path="/" element={<Body />}>
                <Route index element={<Home />} />
                <Route path="/login" element={<Login />} />
                <Route path="/post/:post_id" element={<Article />} />
              </Route>

              {!isAuthenticated ? (
                <Route path="/" element={<BodyPrivate />}>
                  <Route path="/dashboard/home" element={<></>} />
                  <Route path="/dashboard/create-post" element={<FormPost />} />
                  <Route path="/dashboard/list-posts" element={<ListPosts />} />
                </Route>
              ) : (
                <></>
              )}
            </Routes>
          </BrowserRouter>
        </>

      </ConfigProvider>
    </>
  );
};

export default App;