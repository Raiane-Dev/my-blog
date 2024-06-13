import { useEffect, useRef, useState } from 'react';
import apiService from "../services/apiService"

import { SearchOutlined, DeleteOutlined, SmileOutlined, MehOutlined } from '@ant-design/icons';
import type { TableColumnType } from 'antd';
import { Button, Input, Space, Table, Row, Col, notification } from 'antd';
import type { FilterDropdownProps } from 'antd/es/table/interface';
import Highlighter from 'react-highlight-words';


let i = 0;
interface Client {
    id?: number;
    name: string,
    email: string,
    phone: string,
    coordinate: any,
}
type DataIndex = keyof Client;

const ListPosts = () => {
    const [data, setData] = useState([]);
    const [mutex, setMutex] = useState(0);

    const [searchText, setSearchText] = useState('');
    const [searchedColumn, setSearchedColumn] = useState('');
    const searchInput = useRef(null);

    const handleSearch = (
        selectedKeys: string[],
        confirm: FilterDropdownProps['confirm'],
        dataIndex: DataIndex,
    ) => {
        confirm();
        setSearchText(selectedKeys[0]);
        setSearchedColumn(dataIndex);
    };

    const handleReset = (clearFilters: () => void) => {
        clearFilters();
        setSearchText('');
    };

    const handleDelete = (record: any) => {
        apiService.del(
            "/post/" + record.id,
        ).then((response: any) => {
            setMutex(i++)
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


    const search = (dataIndex: any): TableColumnType<any> => ({
        filterDropdown: ({ setSelectedKeys, selectedKeys, confirm, clearFilters, close }) => (
            <div style={{ padding: 8 }} onKeyDown={(e) => e.stopPropagation()}>
                <Input
                    ref={searchInput}
                    placeholder={`Search ${dataIndex}`}
                    value={selectedKeys[0]}
                    onChange={(e) => setSelectedKeys(e.target.value ? [e.target.value] : [])}
                    onPressEnter={() => handleSearch(selectedKeys as string[], confirm, dataIndex)}
                    style={{ marginBottom: 8, display: 'block' }}
                />
                <Space>
                    <Button
                        type="primary"
                        onClick={() => handleSearch(selectedKeys as string[], confirm, dataIndex)}
                        icon={<SearchOutlined />}
                        size="small"
                        style={{ width: 90 }}
                    >
                        Search
                    </Button>
                    <Button
                        onClick={() => clearFilters && handleReset(clearFilters)}
                        size="small"
                        style={{ width: 90 }}
                    >
                        Reset
                    </Button>
                    <Button
                        type="link"
                        size="small"
                        onClick={() => {
                            confirm({ closeDropdown: false });
                            setSearchText((selectedKeys as string[])[0]);
                            setSearchedColumn(dataIndex);
                        }}
                    >
                        Filter
                    </Button>
                    <Button
                        type="link"
                        size="small"
                        onClick={() => {
                            close();
                        }}
                    >
                        close
                    </Button>
                </Space>
            </div>
        ),
        filterIcon: (filtered: boolean) => (
            <SearchOutlined style={{ color: filtered ? '#1677ff' : undefined }} />
        ),
        onFilter: (value, record) =>
            record[dataIndex]
                .toString()
                .toLowerCase()
                .includes((value as string).toLowerCase()),
        render: (text) =>
            searchedColumn === dataIndex ? (
                <Highlighter
                    highlightStyle={{ backgroundColor: '#ffc069', padding: 0 }}
                    searchWords={[searchText]}
                    autoEscape
                    textToHighlight={text ? text.toString() : ''}
                />
            ) : (
                text
            ),
    });

    useEffect(() => {
        apiService.get("/posts")
            .then(response => {
                setData(response.data);
            })
            .catch(err => {
                console.log(err)
            });
    }, [mutex]);

    const columns: any = [
        {
            title: 'Title',
            dataIndex: 'title',
            key: 1,
            ...search('body'),
        },
        {
            title: 'Description',
            dataIndex: 'description',
            key: 3,
            ...search('description'),
        },
        {
            title: '#',
            dataIndex: '#',
            key: 2,
            render: (_: any, record: any) => (
                <Row className='mr-1'>
                    <Col offset={2}>
                        <Button icon={<DeleteOutlined />} onClick={() => handleDelete(record)} ></Button>
                    </Col>
                </Row>
            ),
        }
    ];

    return (
        <>
            <Space direction="vertical" size="middle" style={{ display: 'flex' }}>
                <Table
                    columns={columns}
                    dataSource={data}
                    rowKey="index"
                    pagination={false}

                />
            </Space>
        </>
    );
};

export default ListPosts;
