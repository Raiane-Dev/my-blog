import { Key, ReactNode } from 'react';
import { Layout, Menu, MenuProps } from 'antd';
import { AlertOutlined, InboxOutlined, HomeOutlined } from '@ant-design/icons';
import { Link } from 'react-router-dom';

type MenuItem = Required<MenuProps>['items'][number];

function getItem(
  label: ReactNode,
  key?: Key | null,
  icon?: ReactNode,
  children?: MenuItem[],
  theme?: 'light' | 'dark',
): MenuItem {
  return {
    key,
    icon,
    children,
    label,
    theme,
  } as MenuItem;
}

const items: MenuItem[] = [
  getItem(
    <Link to="/">Home</Link>,
    'home',
    <HomeOutlined />
  ),
  getItem(
    <Link to="/dashboard/create-post">New Post</Link>,
    'create_post',
    <AlertOutlined />
  ),
  getItem(
    <Link to="/dashboard/list-posts">List Posts</Link>,
    'list_clients',
    <InboxOutlined />
  ),
];

const Sidebar = () => {

  return (
    <>
      <Layout.Sider
        breakpoint="lg"
        collapsedWidth="0"
        className='sider-pattern'
        theme='light'
      >
        <Menu
          theme="light"
          mode="inline"
          defaultSelectedKeys={['home']}
          items={items}
        />
      </Layout.Sider>
    </>
  );
};

export default Sidebar;
