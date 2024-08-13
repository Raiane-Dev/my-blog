import { PostInput } from '../models/post';
import { post } from "../services/api_service"
import { Button, Form, Input, Row, notification } from 'antd';
import { SmileOutlined, MehOutlined } from '@ant-design/icons';
import { CKEditor } from '@ckeditor/ckeditor5-react';
import ClassicEditor from '@ckeditor/ckeditor5-build-classic';
import type { UploadProps } from 'antd';
import { InboxOutlined } from '@ant-design/icons';
import { message, Upload } from 'antd';
import UploadAdapter from '../services/uploader';
const { Dragger } = Upload;


const custom_config = {
    extraPlugins: [MyCustomUploadAdapterPlugin],
    toolbar: {
        items: [
            'heading',
            '|',
            'bold',
            'italic',
            'link',
            'bulletedList',
            'numberedList',
            '|',
            'blockQuote',
            'insertTable',
            '|',
            'imageUpload',
            'undo',
            'redo'
        ]
    },
    table: {
        contentToolbar: ['tableColumn', 'tableRow', 'mergeTableCells']
    }
}

function MyCustomUploadAdapterPlugin(editor: any) {
    editor.plugins.get("FileRepository").createUploadAdapter = (loader: any) => {
        return new UploadAdapter(loader, "/api/v1/upload");
    }
}


const FormPost = () => {
    const [form] = Form.useForm();

    const props: UploadProps = {
        name: 'file',
        multiple: false,
        action: '/api/v1/upload',
        onChange(info) {

            const { status } = info.file;
            if (status !== 'uploading') {
                console.log(info.file, info.fileList);
            }
            if (status === 'done') {
                form.setFieldsValue({ image_path: info.file.name });
                message.success(`${info.file.name} file uploaded successfully.`);
            } else if (status === 'error') {
                message.error(`${info.file.name} file upload failed.`);
            }
        },
        onDrop(e) {
            console.log('Dropped file', e.dataTransfer.files);
        },
    };

    const onFinish = (values: any) => {
        post(
            "/new-post",
            values,
        ).then((response: any) => {
            notification.open({
                message: response.statusText,
                icon: <SmileOutlined style={{ color: '#108ee9' }} />,
            });

        })
            .catch(err => {
                notification.open({
                    message: err.response?.statusText ?? "Unable to create",
                    icon: <MehOutlined />,
                });
            })
    };

    return (
        <>
            <Row className="wrapper-form">
                <Form
                    name="basic"
                    className=''
                    form={form}
                    layout="vertical"
                    onFinish={onFinish}
                    autoComplete="off"
                >
                    <Form.Item<PostInput>
                        name="title"
                        label="Title"
                        className='input-pattern w100'
                        rules={[
                            {
                                required: true,
                                message: 'Please enter a name!',
                            },
                        ]}
                    >
                        <Input
                            className='w100'
                        />
                    </Form.Item>

                    <Form.Item<PostInput>
                        name="description"
                        label="Description"
                        className='input-pattern w100'
                    >
                        <Input.TextArea
                            className='w100'
                        />
                    </Form.Item>

                    <Form.Item<Body>
                        name="body"
                        label="Body"
                        className='input-pattern w100'
                        rules={[
                            {
                                required: true,
                                message: 'Please enter an text!',
                            },
                        ]}
                    >
                        <CKEditor
                            editor={ClassicEditor}
                            data=""
                            config={custom_config}
                            onChange={(_: any, editor: any) => {
                                const data = editor.getData();
                                form.setFieldsValue({ body: data });
                            }}
                        />
                    </Form.Item>

                    <Form.Item
                        name="image_path"
                        label="Image"
                        className='w100 end'
                    >
                        <Dragger {...props}>
                            <p className="ant-upload-drag-icon">
                                <InboxOutlined />
                            </p>
                            <p className="ant-upload-text">Click or drag file to this area to upload</p>
                        </Dragger>
                    </Form.Item>

                    <Form.Item
                        className='w100 end'
                    >
                        <Button
                            className='w25 mt-1'
                            type="primary"
                            htmlType="submit"
                        >
                            Criar
                        </Button>
                    </Form.Item>
                </Form>
            </Row>
        </>
    );
};

export default FormPost;
