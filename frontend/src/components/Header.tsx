import { Layout, Typography, Row, Col, Button, Space } from 'antd';
import { InstagramOutlined, TwitterOutlined, GithubOutlined, LoginOutlined } from '@ant-design/icons';

const { Header } = Layout;

const header = () => {

  return (
    <>
      <Layout>
        <Header className='header'>
          <Row>
            <Col span={8} xs={0} sm={8}></Col>

            <Col span={8} xs={20} sm={8}>
              <a href="/">
                <Typography.Title>
                  Raiane Dev
                </Typography.Title>
              </a>
            </Col>

            <Col span={8} xs={20} sm={8} className='fcenter icons-header'>
              <Space align='center'>
                <a href="/login">
                  <Button type="default" icon={<LoginOutlined />} size="large" />
                </a>
                <a href="https://www.instagram.com/raiane_dev/" rel="noreferrer" target='_blank'>
                  <Button type="default" icon={<InstagramOutlined />} size="large" />
                </a>
                <a href="https://twitter.com/RaianeDev" rel="noreferrer"  target='_blank'>
                  <Button type="default" icon={<TwitterOutlined />} size="large" />
                </a>

                <a href="https://github.com/Raiane-Dev" rel="noreferrer"  target='_blank'>
                  <Button type="default" icon={<GithubOutlined />} size="large" />
                </a>
              </Space>
            </Col>

          </Row>

        </Header>
      </Layout>
    </>
  );
};

export default header;
