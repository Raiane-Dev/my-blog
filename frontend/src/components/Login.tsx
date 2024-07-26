import { post } from "../services/api_service"
import { Button, Form, Input, Layout, Space, Typography, notification } from 'antd';
import { SmileOutlined, MehOutlined } from '@ant-design/icons';
import AccountInput from '../models/account';

const { Content } = Layout;

const Login = () => {
    const onFinish = (values: any) => {
        post(
            "/login",
            values
        ).then((response: any) => {
            notification.open({
                message: response.data.message,
                icon: <SmileOutlined style={{ color: '#108ee9' }} />,
              });

            window.location.href = response.data.location ?? "/dashboard/home"
        })
        .catch(err => {
            notification.open({
                message: err.response.data.message ?? "Not exists",
                icon: <MehOutlined />,
              });
        })
    };

    return (
        <>
            <Space direction="vertical" style={{ width: '100%' }}>
                <Layout>
                    <Content className='w100 h100 alg-center jst-center login'>
                        <Form
                            name="basic"
                            className='component-login'
                            layout="vertical"
                            onFinish={onFinish}
                            autoComplete="on"
                        >

                        <Typography.Title>
                            Sign In
                        </Typography.Title>

                            <Form.Item<AccountInput>
                                name="email"
                                label="Email"
                                rules={[
                                    {
                                      type: 'email',
                                      message: 'The input is not valid E-mail!',
                                    },
                                    {
                                      required: true,
                                      message: 'Please input your E-mail!',
                                    },
                                  ]}
                            >
                                <Input />
                            </Form.Item>
                            <Form.Item<AccountInput>
                                name="plain_password"
                                label="Password"
                            >
                                <Input.Password />
                            </Form.Item>

                            <Form.Item>
                                <Button
                                    className='w50'
                                    type="primary"
                                    htmlType="submit"
                                >
                                    Login
                                </Button>

                            </Form.Item>
                        </Form>

                    </Content>
                </Layout>
                <Layout>
                </Layout>
            </Space>
        </>
    );
};

export default Login;
