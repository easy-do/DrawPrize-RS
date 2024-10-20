import React, { useState, useEffect, useMemo } from 'react';
import {
  Table,
  Card,
  PaginationProps,
  Typography,
} from '@arco-design/web-react';
import useLocale from '@/utils/useLocale';
import SearchForm from './form';
import locale from './locale';
import { DefaultSorter, getColumns } from './constants';
import { userDrawHistoryPage } from '@/api/prizeDrawHistory';

import { v4 } from 'uuid';
import InfoPage from './infoPage';

const { Title } = Typography;

function PrizePoolItemPage(props: { prizePoolId: number }) {
  const t = useLocale(locale);

  //表格列回调函数
  const tableCallback = async (record, type) => {
    //查看
    if (type === 'view') {
      viewInfo(record);
    }
  };


  //查看
  const [viewVisible, setViewVisibled] = useState(false);
  const [viewInfoData, setViewInfoData] = useState();
  function viewInfo(record) {
    setViewInfoData(record);
    setViewVisibled(true);
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
    props.prizePoolId,
  ]);

  //分页请求
  function fetchData() {
    const { current, pageSize } = pagination;
    setLoading(true);
    userDrawHistoryPage({
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
    setFormParams(params);
    setSearchId(v4());
  }

  return (
    <Card>
      <Title heading={6}>{t['menu.list.searchTable']}</Title>
      <SearchForm prizePoolId={props.prizePoolId} onSearch={handleSearch} />
      <Table
        rowKey="id"
        loading={loading}
        onChange={onChangeTable}
        pagination={pagination}
        columns={columns}
        data={data}
      />
      <InfoPage
        info={viewInfoData}
        visible={viewVisible}
        setVisible={setViewVisibled}
      />
    </Card>
  );
}

export default PrizePoolItemPage;
