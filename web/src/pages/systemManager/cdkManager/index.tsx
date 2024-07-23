import React, { useState, useEffect, useMemo } from 'react';
import {
  Table,
  Card,
  PaginationProps,
  Typography,
  Space,
  Button,
  Notification,
  Popconfirm,
} from '@arco-design/web-react';
import useLocale from '@/utils/useLocale';
import SearchForm from './form';
import locale from './locale';
import { DefaultSorter, getColumns } from './constants';
import { exportCdk, getCdkPage, removeCdk } from '@/api/cdk';
import styles from './style/index.module.less';

import { v4 } from 'uuid';
import UpdatePage from './updatePage';
import PermissionWrapper from '@/components/PermissionWrapper';
import {
  IconDelete,
  IconDownload,
  IconPlus,
} from '@arco-design/web-react/icon';
import AddPage from './addPage';

const { Title } = Typography;

function SearchTable() {
  const t = useLocale(locale);

  //表格列回调函数
  const tableCallback = async (record, type) => {
    //新增
    if (type === 'add') {
      addData();
    }
    //编辑
    if (type === 'update') {
      updateData(record.id);
    }
    //删除
    if (type === 'delete') {
      deleteData([record.id]);
    }
    //删除
    if (type === 'deletes') {
      deleteData(selectRows.map((item) => item.id));
    }
    //导出
    if (type === 'export') {
      exportData();
    }
  };

  //添加
  const [addVisible, setAddVisible] = useState(false);
  function addData() {
    setAddVisible(true);
  }

  //编辑
  const [updateVisible, setUpdateVisibled] = useState(false);
  const [updateInfoId, setUpdateInfoId] = useState();
  function updateData(id) {
    setUpdateInfoId(id);
    setUpdateVisibled(true);
  }

  //删除
  function deleteData(ids) {
    removeCdk(ids).then((res) => {
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
  //导出
  function exportData() {
    exportCdk(selectRows.map((item) => item.id)).then((res) => {
      const { success, message, data } = res.data;
      if (success) {
        const blob = new Blob([data]);
        const objectURL = URL.createObjectURL(blob);
        const btn = document.createElement('a');
        btn.download = 'CDK导出.txt';
        btn.href = objectURL;
        btn.click();
        URL.revokeObjectURL(objectURL);
        fetchData();
      } else {
        Notification.error({ content: message, duration: 1000 });
        fetchData();
      }
    });
  }

  //选中的列
  const [selectRows, setSelectRows] = useState([]);

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
    getCdkPage({
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

    if (params.useStatus != undefined && 'string' == typeof params.useStatus) {
      params.useStatus = params.useStatus == 'true';
    }
    setFormParams(params);
    setSearchId(v4());
  }

  return (
    <Card>
      <Title heading={6}>{t['menu.list.searchTable']}</Title>
      <SearchForm onSearch={handleSearch} />
      <Space>
        <PermissionWrapper
          requiredPermissions={[
            { resource: 'cdk_manager', actions: ['api_cdk_add'] },
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
        <PermissionWrapper
          requiredPermissions={[
            { resource: 'cdk_manager', actions: ['api_cdk_export'] },
          ]}
        >
          <div className={styles['button-group']}>
            <Space>
              <Button
                disabled={selectRows.length == 0}
                type="primary"
                icon={<IconDownload />}
                onClick={() => tableCallback(null, 'export')}
              >
                {t['searchTable.operations.export']}
              </Button>
            </Space>
          </div>
        </PermissionWrapper>
        <PermissionWrapper
          requiredPermissions={[
            { resource: 'cdk_manager', actions: ['api_cdk_delete'] },
          ]}
        >
          <Popconfirm
            focusLock
            title={t['option.delete.confirm.title']}
            onOk={() => {
              tableCallback(null, 'deletes');
            }}
          >
            <div className={styles['button-group']}>
              <Space>
                <Button
                  status="danger"
                  disabled={selectRows.length == 0}
                  type="primary"
                  icon={<IconDelete />}
                >
                  {t['searchTable.columns.operations.delete']}
                </Button>
              </Space>
            </div>
          </Popconfirm>
        </PermissionWrapper>
      </Space>

      <Table
        rowSelection={{
          checkAll: true,
          onChange: (_selectedRowKeys, selectedRows) => {
            setSelectRows(selectedRows);
          },
        }}
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
