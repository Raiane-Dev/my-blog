import {  FC } from 'react';
import { ConfigProvider, Layout } from 'antd';
import { Routes, Route } from 'react-router-dom';
import pt_br from 'antd/locale/pt_BR'
import './App.less'
import Home from './components/Home';
import Header from './components/Header';
import Footer from './components/Footer';
import Article from './components/Article';

const App: FC = () => {

  return (
    <>
      <ConfigProvider locale={pt_br}>
                <>
                <Layout className='layout-pattern'>
                  <Header />
                    <Layout>
                        <Layout.Content className='content-pattern'>
                      <Routes>
                        <Route path="/home" element={<Home />} />
                        <Route path="/post/:post_id" element={<Article />} />
                      </Routes>
                    </Layout.Content>
                    <Footer />
                  </Layout>
                </Layout>
                </>
      </ConfigProvider>
    </>
  );
};

export default App;
