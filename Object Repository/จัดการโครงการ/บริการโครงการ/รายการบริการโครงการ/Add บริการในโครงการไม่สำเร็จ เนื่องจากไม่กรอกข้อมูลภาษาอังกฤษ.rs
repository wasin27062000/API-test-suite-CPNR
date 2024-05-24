<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Add บริการในโครงการไม่สำเร็จ เนื่องจากไม่กรอกข้อมูลภาษาอังกฤษ</name>
   <tag></tag>
   <elementGuidId>e6bf5664-9fa7-45ba-9dd1-4e4b9c7596bd</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n   \&quot;id\&quot;:null,\n   \&quot;name\&quot;:{\n      \&quot;th\&quot;:\&quot;ทดสอบ\&quot;,\n      \&quot;en\&quot;:\&quot;\&quot;\n   },\n   \&quot;shortDescription\&quot;:{\n      \&quot;th\&quot;:\&quot;ทดสอบ\&quot;,\n      \&quot;en\&quot;:\&quot;\&quot;\n   },\n   \&quot;description\&quot;:{\n      \&quot;th\&quot;:\&quot;\u003cp\u003eทดสอบ\u003c/p\u003e\&quot;,\n      \&quot;en\&quot;:\&quot;\&quot;\n   },\n   \&quot;createdAt\&quot;:null,\n   \&quot;image\&quot;:\&quot;\&quot;,\n   \&quot;category\&quot;:{\n      \&quot;id\&quot;:\&quot;3a7b407e-3a37-4a04-b344-42c151fc1ae5\&quot;,\n      \&quot;createdAt\&quot;:\&quot;2023-05-17T04:38:32.923Z\&quot;,\n      \&quot;updatedAt\&quot;:\&quot;2023-10-17T08:48:00.000Z\&quot;,\n      \&quot;name\&quot;:{\n         \&quot;th\&quot;:\&quot;ทดสอบหมวดหมู่ 2\&quot;,\n         \&quot;en\&quot;:\&quot;\&quot;\n      }\n   },\n   \&quot;buttonType\&quot;:\&quot;phone\&quot;,\n   \&quot;buttonValue\&quot;:\&quot;\&quot;,\n   \&quot;buttonName\&quot;:{\n      \&quot;th\&quot;:\&quot;โทร\&quot;,\n      \&quot;en\&quot;:\&quot;\&quot;\n   },\n   \&quot;active\&quot;:false\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>c019f92c-9f4a-4065-8723-abc60d65d471</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>bearer ${token}</value>
      <webElementGuid>b4ae9433-e4e9-4928-9050-fa139cb53cc8</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.4.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>http://45.150.131.146:7090/api-backoffice/project/a56b8623-1b44-4da8-9a19-d4479981c8fa/service</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>024afb54-7036-439f-a5c1-20eefc53d5fc</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
