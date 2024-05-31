import { Layout, Typography, Row, Col, Button, Space } from 'antd';
import { InstagramOutlined, TwitterOutlined, GithubOutlined, LinkedinOutlined } from '@ant-design/icons';

const { Header, Content } = Layout;

const header = () => {

  return (
    <>
      <Layout>
        <Header className='header'>
          <Row>
            <Col span={8}>

            </Col>

            <Col span={8}>
              <Typography.Title>
                Raiane Dev
              </Typography.Title>
            </Col>

            <Col span={8} className='fcenter'>
              <Space align='center'>
                <a href="https://www.instagram.com/raiane_dev/" target='_blank'>
                  <Button type="default" icon={<InstagramOutlined />} size="large" />
                </a>
                <a href="https://twitter.com/RaianeDev" target='_blank'>
                  <Button type="default" icon={<TwitterOutlined />} size="large" />
                </a>

                <a href="https://github.com/Raiane-Dev" target='_blank'>
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
