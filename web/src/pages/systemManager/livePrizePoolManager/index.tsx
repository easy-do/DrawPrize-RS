import React, { useState, useEffect, useMemo } from 'react';
import {
  Table,
  Card,
  PaginationProps,
  Typography,
  Modal,
} from '@arco-design/web-react';
import useLocale from '@/utils/useLocale';
import SearchForm from './form';
import locale from './locale';
import { DefaultSorter, getColumns } from './constants';
import { getLivePrizePoolPage } from '@/api/livePrizePool';

import { v4 } from 'uuid';
import InfoPage from './infoPage';
import UpdatePage from './updatePage';
import LivePrizePoolItemPage from './livePrizePoolItem';

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
    //奖品管理
    if (type === 'item_manager') {
      itemManager(record.id);
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


  //奖品管理
  const [itemManagerVisible, setItemManagerVisibled] = useState(false);
  const [livePrizePoolId, setLivePrizePoolId] = useState();
  function itemManager(id) {
    setLivePrizePoolId(id);
    setItemManagerVisibled(true);
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
    getLivePrizePoolPage({
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
    if (
      params.share_pool != undefined &&
      'string' == typeof params.share_pool
    ) {
      params.share_pool = params.share_pool == 'true';
    }
    setFormParams(params);
    setSearchId(v4());
  }

  return (
    <Card>
      <Title heading={6}>{t['menu.list.searchTable']}</Title>
      <SearchForm onSearch={handleSearch} />
      <Table
        rowKey="id"
        loading={loading}
        onChange={onChangeTable}
        pagination={pagination}
        columns={columns}
        data={data}
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
        title={t['searchTable.columns.operations.item_manager']}
        style={{ minWidth: '95%', minHeight: '90%' }}
        visible={itemManagerVisible}
        onCancel={() => setItemManagerVisibled(false)}
        footer={null}
        maskClosable={false}
      >
        <LivePrizePoolItemPage livePrizePoolId={livePrizePoolId} />
      </Modal>
    </Card>
  );
}

export default SearchTable;
