import React, { useState, useEffect, useMemo } from 'react';
import {
  Table,
  Card,
  PaginationProps,
  Button,
  Space,
  Typography,
  Notification,
} from '@arco-design/web-react';
import PermissionWrapper from '@/components/PermissionWrapper';
import { IconPlus } from '@arco-design/web-react/icon';
import useLocale from '@/utils/useLocale';
import SearchForm from './form';
import locale from './locale';
import styles from './style/index.module.less';
import { DefaultSorter, getColumns } from './constants';
import { getResourcePage, removeResource } from '@/api/resource';

import { v4 } from 'uuid';
import AddPage from './addPage';
import InfoPage from './infoPage';
import UpdatePage from './updatePage';

const { Title } = Typography;

function SearchTable() {
  const t = useLocale(locale);

  //表格列回调函数
  const tableCallback = async (record, type) => {
    //新增
    if (type === 'add') {
      addData();
    }
    //查看
    if (type === 'view') {
      viewInfo(record.id);
    }
    //编辑
    if (type === 'update') {
      updateData(record.id);
    }
    //删除
    if (type === 'delete') {
      deleteData(record.id);
    }
  };

  //添加
  const [addVisible, setAddVisible] = useState(false);
  function addData() {
    setAddVisible(true);
  }

  //查看
  const [viewVisible, setViewVisibled] = useState(false);
  const [viewInfoId, setViewInfoId] = useState();
  function viewInfo(id) {
    setViewInfoId(id);
    setViewVisibled(true);
  }

  //编辑
  const [updateVisible, setUpdateVisibled] = useState(false);
  const [updateInfoId, setUpdateInfoId] = useState();
  function updateData(id) {
    setUpdateInfoId(id);
    setUpdateVisibled(true);
  }

  //删除
  function deleteData(id) {
    removeResource(id).then((res) => {
      const { success, message } = res.data;
      if (success) {
        Notification.success({ content: message, duration: 1000 });
        fetchData();
      } else {
        Notification.error({ content: message, duration: 1000 });
        fetchData();
      }
    });
  }

  const columns = useMemo(() => getColumns(t, tableCallback), [t]);

  const [data, setData] = useState([]);
  const [pagination, setPatination] = useState<PaginationProps>({
    sizeCanChange: true,
    showTotal: true,
    pageSize: 10,
    current: 1,
    pageSizeChangeResetCurrent: true,
  });
  const [sorter, setSorter] = useState(DefaultSorter);
  const [loading, setLoading] = useState(true);
  const [formParams, setFormParams] = useState<any>({});
  const [searchId, setSearchId] = useState<string>('');

  useEffect(() => {
    fetchData();
  }, [
    pagination.current,
    pagination.pageSize,
    JSON.stringify(sorter),
    JSON.stringify(formParams),
    searchId,
  ]);

  //分页请求
  function fetchData() {
    const { current, pageSize } = pagination;
    setLoading(true);
    getResourcePage({
      page_data: {
        page: current,
        page_size: pageSize,
        sorter,
      },
      ...formParams,
    }).then((res) => {
      const { total, record } = res.data.data;
      setData(record);
      setPatination({
        ...pagination,
        current,
        pageSize,
        total: total,
      });
      setLoading(false);
    });
  }

  //表格分页回调
  function onChangeTable({ current, pageSize }, { field, direction }) {
    setPatination({
      ...pagination,
      current,
      pageSize,
    });

    if (direction == undefined) {
      setSorter(undefined);
    } else {
      setSorter({
        field,
        direction: direction === 'ascend' ? 'asc' : 'desc',
      });
    }
  }

  //搜索表单回调
  function handleSearch(params) {
    setPatination({ ...pagination, current: 1 });
    if (
      params.resource_root != undefined &&
      'string' == typeof params.resource_root
    ) {
      params.resource_root = params.resource_root == 'true';
    }
    if (
      params.resource_action != undefined &&
      'string' == typeof params.resource_action
    ) {
      params.resource_action = params.resource_action == 'true';
    }
    if (params.status != undefined && 'string' == typeof params.status) {
      params.status = params.status == 'true';
    }
    if (
      params.order_number != undefined &&
      'string' == typeof params.order_number
    ) {
      params.order_number = Number(params.order_number);
    }
    setFormParams(params);
    setSearchId(v4());
  }

  return (
    <Card>
      <Title heading={6}>{t['menu.list.searchTable']}</Title>
      <SearchForm onSearch={handleSearch} />
      <PermissionWrapper
        requiredPermissions={[
          { resource: 'resource_manager', actions: ['api_resource_add'] },
        ]}
      >
        <div className={styles['button-group']}>
          <Space>
            <Button
              type="primary"
              icon={<IconPlus />}
              onClick={() => tableCallback(null, 'add')}
            >
              {t['searchTable.operations.add']}
            </Button>
          </Space>
        </div>
      </PermissionWrapper>
      <Table
        rowKey="id"
        loading={loading}
        onChange={onChangeTable}
        pagination={pagination}
        columns={columns}
        data={data}
      />
      <AddPage
        visible={addVisible}
        setVisible={setAddVisible}
        callback={fetchData}
      />
      <InfoPage
        id={viewInfoId}
        visible={viewVisible}
        setVisible={setViewVisibled}
      />
      <UpdatePage
        id={updateInfoId}
        visible={updateVisible}
        setVisible={setUpdateVisibled}
        callback={fetchData}
      />
    </Card>
  );
}

export default SearchTable;
