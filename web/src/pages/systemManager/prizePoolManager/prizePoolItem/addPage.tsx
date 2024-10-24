import React, { useContext, useEffect, useRef } from 'react';
import {
  Form,
  FormInstance,
  Input,
  InputNumber,
  Modal,
  Notification,
  Select,
  Upload,
} from '@arco-design/web-react';
import locale from './locale';
import useLocale from '@/utils/useLocale';
import { GlobalContext } from '@/context';
import { addPrizePoolItem } from '@/api/prizePoolItem';
import FormItem from '@arco-design/web-react/es/Form/form-item';
import { UploadItem } from '@arco-design/web-react/es/Upload';
import { fileToBase64 } from '@/utils/fileutil';

function AddPage(props: {
  visible;
  setVisible;
  prizePoolId;
  callback: () => void;
}) {
  const formRef = useRef<FormInstance>();

  const { lang } = useContext(GlobalContext);

  const t = useLocale(locale);

  const [loading, setLoading] = React.useState(false);

  const [fileList, setFileList] = React.useState<UploadItem[]>([]);
  const [base64File, setBase64File] = React.useState('');

  const handleSubmit = () => {
    formRef.current.validate().then((values) => {
      setLoading(true);
      values.status = values.status == 'true';
      values.guarantees = values.guarantees == 'true';
      values.icon = base64File;
      addPrizePoolItem(values)
        .then((res) => {
          const { success, message } = res.data;
          if (success) {
            Notification.success({ content: message, duration: 1000 });
            props.setVisible(false);
            props.callback();
          }
        })
        .finally(() => {
          setLoading(false);
        });
    });
  };

  useEffect(() => {
    formRef.current?.setFieldsValue({ pool_id: props.prizePoolId });
  }, [props.visible]);

  return (
    <Modal
      title={t['searchTable.operations.add']}
      style={{ width: '35%' }}
      visible={props.visible}
      onOk={() => {
        handleSubmit();
      }}
      onCancel={() => {
        props.setVisible(false);
      }}
      autoFocus={false}
      focusLock={true}
      confirmLoading={loading}
    >
      <Form
        ref={formRef}
        style={{ width: '95%', marginTop: '6px' }}
        labelCol={{ span: lang === 'en-US' ? 7 : 6 }}
        wrapperCol={{ span: lang === 'en-US' ? 17 : 18 }}
      >
        <FormItem
          disabled
          required
          hidden
          initialValue={props.prizePoolId}
          label={t['searchTable.columns.pool_id']}
          field={'pool_id'}
        >
          <Input placeholder={t['searchForm.placeholder']} allowClear />
        </FormItem>
        <FormItem
          required
          label={t['searchTable.columns.prize_name']}
          field={'prize_name'}
        >
          <Input placeholder={t['searchForm.placeholder']} allowClear />
        </FormItem>
        <FormItem required label={t['searchTable.columns.icon']} field={'icon'}>
          <Upload
              listType='picture-card'
              autoUpload={false}
              limit={1}
              showUploadList={{
                previewIcon : null
              }}
              onChange={(list)=>{
                setFileList(list);
                if (list.length > 0) {
                  fileToBase64(list[0].originFile,(base64)=>{
                    setBase64File(base64)
                  })
                }else{
                  setBase64File('')
                }
              }}
              fileList={fileList}
            />
        </FormItem>
        <FormItem
          required
          label={t['searchTable.columns.level']}
          field={'level'}
        >
          <InputNumber
            min={1}
            type="number"
            placeholder={t['searchForm.placeholder']}
          />
        </FormItem>
        <FormItem
          required
          label={t['searchTable.columns.level_name']}
          field={'level_name'}
        >
          <Input placeholder={t['searchForm.placeholder']} allowClear />
        </FormItem>
        <FormItem
          required
          label={t['searchTable.columns.probability']}
          field={'probability'}
        >
          <Input placeholder={t['searchForm.placeholder']} allowClear />
        </FormItem>
        <FormItem
          required
          label={t['searchTable.columns.quantity']}
          field={'quantity'}
        >
          <InputNumber min={1} placeholder={t['searchForm.placeholder']} />
        </FormItem>
        <FormItem
          required
          label={t['searchTable.columns.prize_desc']}
          field={'prize_desc'}
        >
          <Input placeholder={t['searchForm.placeholder']} allowClear />
        </FormItem>
        <FormItem
          required
          initialValue={'false'}
          label={t['searchTable.columns.guarantees']}
          field={'guarantees'}
        >
          <Select
            placeholder={t['searchForm.placeholder']}
            options={[
              {
                label: t['searchForm.enable'],
                value: 'true',
              },
              {
                label: t['searchForm.disable'],
                value: 'false',
              },
            ]}
            allowClear
          />
        </FormItem>
        <FormItem
          required
          initialValue={'true'}
          label={t['searchTable.columns.status']}
          field={'status'}
        >
          <Select
            placeholder={t['searchForm.placeholder']}
            options={[
              {
                label: t['searchForm.enable'],
                value: 'true',
              },
              {
                label: t['searchForm.disable'],
                value: 'false',
              },
            ]}
            allowClear
          />
        </FormItem>
      </Form>
    </Modal>
  );
}

export default AddPage;
