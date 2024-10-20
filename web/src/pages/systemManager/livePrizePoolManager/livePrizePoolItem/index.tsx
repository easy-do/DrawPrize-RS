import React, { useState, useEffect, useMemo } from 'react';
import {
  Table,
  Card,
  PaginationProps,
  Typography,
  Notification,
  Space,
  Button,
  Modal,
  Form,
  Input
} from '@arco-design/web-react';
import useLocale from '@/utils/useLocale';
import SearchForm from './form';
import locale from './locale';
import { DefaultSorter, getColumns } from './constants';
import { cleanLivePrizePoolItemCdk, getLivePrizePoolItemPage, importLivePrizePoolItemCdk, removeLivePrizePoolItem } from '@/api/livePrizePoolItem';

import { v4 } from 'uuid';
import InfoPage from './infoPage';
import UpdatePage from './updatePage';
import PermissionWrapper from '@/components/PermissionWrapper';
import { IconPlus } from '@arco-design/web-react/icon';
import styles from './style/index.module.less';
import AddPage from './addPage';

const { Title } = Typography;

function PrizePoolItemPage(props: { livePrizePoolId: number }) {
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
    //删除
    if (type === 'import_cdk') {
      importCdk(record.id);
    }
    //删除
    if (type === 'clean_cdk') {
      cleanCdk(record.id);
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
    removeLivePrizePoolItem(id).then((res) => {
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

    //导入cdk
    const [importCdkVisible, setImportCdkVisibled] = useState(false);
    const [importCdkId, setImportCdkId] = useState();
    const [cdktValues, setCdktValues] = useState('');
    function importCdk(id) {
      setImportCdkId(id);
      setImportCdkVisibled(true);
    }

  //清空cdk
  function cleanCdk(id) {
    cleanLivePrizePoolItemCdk(props.livePrizePoolId,id).then((res) => {
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
    props.livePrizePoolId,
  ]);

  //分页请求
  function fetchData() {
    const { current, pageSize } = pagination;
    setLoading(true);
    getLivePrizePoolItemPage({
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

    if (params.status != undefined && 'string' == typeof params.status) {
      params.status = params.status == 'true';
    }
    if (params.guarantees != undefined && 'string' == typeof params.guarantees) {
      params.guarantees = params.guarantees == 'true';
    }
    setFormParams(params);
    setSearchId(v4());
  }

  return (
    <Card>
      <Title heading={6}>{t['menu.list.searchTable']}</Title>
      <SearchForm livePrizePoolId={props.livePrizePoolId} onSearch={handleSearch} />
      <PermissionWrapper
        requiredPermissions={[
          { resource: 'live_prize_pool_item_manager', actions: ['api_live_prize_pool_item_add'] },
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
      livePrizePoolId={props.livePrizePoolId} 
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
      <Modal
        title={t['searchTable.columns.operations.import_cdk']}
        visible={importCdkVisible}
        onCancel={() => setImportCdkVisibled(false)}
        onConfirm={() => {
          importLivePrizePoolItemCdk({
            live_id: props.livePrizePoolId,
            prize_id: importCdkId,
            cdk: cdktValues.split('\n'),
          }).then((res) => {
            const { success, data } = res.data;
            if (success) {
              setImportCdkVisibled(false);
              Notification.success({ content: "成功导入"+data+"个", duration: 1000 });
              fetchData();
            }
          });
        }}
      >
          <Input.TextArea style={{minHeight:'300px'}} defaultValue={cdktValues} onChange={(value)=>setCdktValues(value)}  placeholder='每行一个' allowClear />
      </Modal>
    </Card>
  );
}

export default PrizePoolItemPage;
